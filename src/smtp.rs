use lettre::smtp::client::net::NetworkStream;
use lettre::smtp::client::InnerClient;
use lettre::smtp::commands::*;
use lettre::smtp::error::Error;
use lettre::smtp::extension::ClientId;
use lettre::EmailAddress;
use std::time::Duration;
use trust_dns_resolver::Name;

// Try to send an smtp command, close if fails.
macro_rules! try_smtp (
    ($res: expr, $client: ident, $host: expr, $port: expr) => ({
		if let Err(err) = $res {
			debug!("Closing {}:{}, because of error '{}'.", $host, $port, err);
			$client.close();
			return Err(err);
		}
    })
);

pub fn email_exists(
	from_email: &str,
	to_email: &str,
	host: &Name,
	port: u16,
) -> Result<bool, Error> {
	debug!("Connecting to {}:{}", host, port);
	let mut smtp_client: InnerClient<NetworkStream> = InnerClient::new();
	let timeout = Some(Duration::new(3, 0)); // Set timeout to 3s

	// Set timeout.
	if let Err(err) = smtp_client.set_timeout(timeout) {
		debug!("Closing {}:{}, because of error '{}'.", host, port, err);
		smtp_client.close();
		return Err(Error::from(err));
	}

	// Connect to the host.
	try_smtp!(
		smtp_client.connect(&(host.to_utf8().as_str(), port), timeout, None),
		smtp_client,
		host,
		port
	);

	// Send ehlo.
	try_smtp!(
		smtp_client.command(EhloCommand::new(ClientId::new("localhost".to_string()))),
		smtp_client,
		host,
		port
	);

	// Send from.
	try_smtp!(
		smtp_client.command(MailCommand::new(
			Some(EmailAddress::new(from_email.to_string()).unwrap()),
			vec![],
		)),
		smtp_client,
		host,
		port
	);

	// Send to.
	let result = match smtp_client.command(RcptCommand::new(
		EmailAddress::new(to_email.to_string()).unwrap(),
		vec![],
	)) {
		Ok(response) => match response.first_line() {
			Some(message) => {
				// 250 2.1.5 Recipient e-mail address ok.
				if message.contains("2.1.5") {
					Ok(true)
				} else {
					Err(Error::Client("Can't find 2.1.5 in RCPT command"))
				}
			}
			None => Err(Error::Client("No response on RCPT command")),
		},
		Err(err) => {
			// 550 5.1.1 Mailbox does not exist.
			if err.to_string().contains("5.1.1") {
				Ok(false)
			} else {
				Err(err)
			}
		}
	};

	// Quit.
	smtp_client.close();

	match result {
		Ok(val) => debug!("Checked email on {}:{}, exists={}.", host, port, val),
		Err(_) => debug!("Cannot check email on {}:{}.", host, port),
	};
	result
}
