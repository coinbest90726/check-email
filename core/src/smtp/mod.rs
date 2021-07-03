// check-if-email-exists
// Copyright (C) 2018-2021 Reacher

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod yahoo;

use super::util::{constants::LOG_TARGET, input_output::CheckEmailInput};
use crate::util::ser_with_display::ser_with_display;
use async_smtp::{
	smtp::{
		client::net::NetworkStream, commands::*, error::Error as AsyncSmtpError,
		extension::ClientId,
	},
	ClientSecurity, EmailAddress, SmtpClient, SmtpTransport,
};
use async_std::future;
use fast_socks5::{
	client::{Config, Socks5Stream},
	Result, SocksError,
};
use rand::rngs::SmallRng;
use rand::{distributions::Alphanumeric, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::iter;
use std::str::FromStr;
use std::time::Duration;
use trust_dns_proto::rr::Name;
use yahoo::YahooError;

/// Details that we gathered from connecting to this email via SMTP
#[derive(Debug, Deserialize, Serialize)]
pub struct SmtpDetails {
	/// Are we able to connect to the SMTP server?
	pub can_connect_smtp: bool,
	/// Is this email account's inbox full?
	pub has_full_inbox: bool,
	/// Does this domain have a catch-all email address?
	pub is_catch_all: bool,
	/// Can we send an email to this address?
	pub is_deliverable: bool,
	/// Is the email blocked or disabled by the provider?
	pub is_disabled: bool,
}

impl Default for SmtpDetails {
	fn default() -> Self {
		SmtpDetails {
			can_connect_smtp: false,
			has_full_inbox: false,
			is_catch_all: false,
			is_deliverable: false,
			is_disabled: false,
		}
	}
}

/// Error occured connecting to this email server via SMTP.
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum SmtpError {
	/// Error if we're using a SOCKS5 proxy.
	#[serde(serialize_with = "ser_with_display")]
	SocksError(SocksError),
	/// Error when communicating with SMTP server.
	#[serde(serialize_with = "ser_with_display")]
	SmtpError(AsyncSmtpError),
	/// Time-out error.
	#[serde(serialize_with = "ser_with_display")]
	TimeoutError(future::TimeoutError),
	/// Error when verifying a Yahoo email.
	YahooError(YahooError),
}

impl From<AsyncSmtpError> for SmtpError {
	fn from(error: AsyncSmtpError) -> Self {
		SmtpError::SmtpError(error)
	}
}

impl From<SocksError> for SmtpError {
	fn from(error: SocksError) -> Self {
		SmtpError::SocksError(error)
	}
}

impl From<future::TimeoutError> for SmtpError {
	fn from(error: future::TimeoutError) -> Self {
		SmtpError::TimeoutError(error)
	}
}

impl From<YahooError> for SmtpError {
	fn from(error: YahooError) -> Self {
		SmtpError::YahooError(error)
	}
}

/// Try to send an smtp command, close and return Err if fails.
macro_rules! try_smtp (
    ($res: expr, $client: ident, $host: expr, $port: expr) => ({
		if let Err(err) = $res {
			log::debug!(target: LOG_TARGET, "Closing {}:{}, because of error '{}'.", $host, $port, err);
			$client.close().await?;

			return Err(err.into());
		}
    })
);

/// Attempt to connect to host via SMTP, and return SMTP client on success.
async fn connect_to_host(
	host: &Name,
	port: u16,
	input: &CheckEmailInput,
) -> Result<SmtpTransport, SmtpError> {
	let mut smtp_client =
		SmtpClient::with_security((host.to_utf8().as_ref(), port), ClientSecurity::None)
			.await?
			// FIXME Do not clone?
			.hello_name(ClientId::Domain(input.hello_name.clone()))
			.timeout(Some(Duration::new(30, 0))) // Set timeout to 30s
			.into_transport();

	// Connect to the host. If the proxy argument is set, use it.
	log::debug!(target: LOG_TARGET, "Connecting to {}:{}", host, port);
	if let Some(proxy) = &input.proxy {
		let stream = Socks5Stream::connect(
			(proxy.host.as_ref(), proxy.port),
			host.to_utf8(),
			port,
			Config::default(),
		)
		.await?;

		try_smtp!(
			smtp_client
				.connect_with_stream(NetworkStream::Socks5Stream(stream))
				.await,
			smtp_client,
			host,
			port
		);
	} else {
		try_smtp!(smtp_client.connect().await, smtp_client, host, port);
	}

	// "MAIL FROM: user@example.org"
	let from_email = EmailAddress::from_str(input.from_email.as_ref()).unwrap_or_else(|_| {
		log::warn!(
			"Inputted from_email \"{}\" is not a valid email, using \"user@example.org\" instead",
			input.from_email
		);
		EmailAddress::from_str("user@example.org").expect("This is a valid email. qed.")
	});
	try_smtp!(
		smtp_client
			// FIXME Do not clone?
			.command(MailCommand::new(Some(from_email.clone()), vec![],))
			.await,
		smtp_client,
		host,
		port
	);

	Ok(smtp_client)
}

/// Description of the deliverability information we can gather from
/// communicating with the SMTP server.
struct Deliverability {
	/// Is this email account's inbox full?
	has_full_inbox: bool,
	/// Can we send an email to this address?
	is_deliverable: bool,
	/// Is the email blocked or disabled by the provider?
	is_disabled: bool,
}

/// Check if `to_email` exists on host SMTP server. This is the core logic of
/// this tool.
async fn email_deliverable(
	smtp_client: &mut SmtpTransport,
	to_email: &EmailAddress,
) -> Result<Deliverability, SmtpError> {
	// "RCPT TO: me@email.com"
	// FIXME Do not clone?
	match smtp_client
		.command(RcptCommand::new(to_email.clone(), vec![]))
		.await
	{
		Ok(_) => {
			// According to RFC 5321, `RCPT TO` command succeeds with 250 and
			// 251 codes only (no 3xx codes at all):
			// https://tools.ietf.org/html/rfc5321#page-56
			//
			// Where the 251 code is used for forwarding, which is not our case,
			// because we always deliver to the SMTP server hosting the address
			// itself.
			//
			// So, if `response.is_positive()` (which is a condition for
			// returning `Ok` from the `command()` method above), then delivery
			// succeeds, accordingly to RFC 5321.
			Ok(Deliverability {
				has_full_inbox: false,
				is_deliverable: true, // response.is_positive()
				is_disabled: false,
			})
		}
		Err(err) => {
			// We cast to lowercase, because our matched strings below are all
			// lowercase.
			let err_string = err.to_string().to_lowercase();

			// Check if the email account has been disabled or blocked.
			// 554 The email account that you tried to reach is disabled. Learn more at https://support.google.com/mail/?p=DisabledUser"
			if err_string.contains("disabled")
				// 554 delivery error: Sorry your message to [email] cannot be delivered. This account has been disabled or discontinued
				|| err_string.contains("discontinued")
			{
				return Ok(Deliverability {
					has_full_inbox: false,
					is_deliverable: false,
					is_disabled: true,
				});
			}

			// Check if the email account has a full inbox.
			if err_string.contains("full")
				|| err_string.contains("insufficient")
				|| err_string.contains("over quota")
				|| err_string.contains("space")
				// 550 user has too many messages on the server
				|| err_string.contains("too many messages")
			{
				return Ok(Deliverability {
					has_full_inbox: true,
					is_deliverable: false,
					is_disabled: false,
				});
			}

			// Check error messages that say that user can actually receive
			// emails.
			// 4.2.1 The user you are trying to contact is receiving mail at a rate that
			if err_string
				.contains("the user you are trying to contact is receiving mail at a rate that")
			{
				return Ok(Deliverability {
					has_full_inbox: false,
					is_deliverable: true,
					is_disabled: false,
				});
			}

			// These are the possible error messages when email account doesn't exist.
			// 550 Address rejected
			// 550 5.1.1 : Recipient address rejected
			// 550 5.1.1 : Recipient address rejected: User unknown in virtual alias table
			// 550 5.1.1 <user@domain.com>: Recipient address rejected: User unknown in relay recipient table
			if err_string.contains("address rejected")
				// 550 5.1.1 : Unrouteable address
				|| err_string.contains("unrouteable")
				// 550 5.1.1 : The email account that you tried to reach does not exist
				|| err_string.contains("does not exist")
				// 550 invalid address
				// 550 User not local or invalid address – Relay denied
				|| err_string.contains("invalid address")
				// 5.1.1 Invalid email address
				|| err_string.contains("invalid email address")
				// 550 Invalid recipient
				|| err_string.contains("invalid recipient")
				|| err_string.contains("may not exist")
				|| err_string.contains("recipient invalid")
				// 550 5.1.1 : Recipient rejected
				|| err_string.contains("recipient rejected")
				|| err_string.contains("undeliverable")
				// 550 User unknown
				// 550 5.1.1 <EMAIL> User unknown
				// 550 recipient address rejected: user unknown in local recipient table
				|| err_string.contains("user unknown")
				// 550 Unknown user
				|| err_string.contains("unknown user")
				// 5.1.1 Recipient unknown <EMAIL>
				|| err_string.contains("recipient unknown")
				// 550 5.1.1 No such user - pp
				// 550 No such user here
				|| err_string.contains("no such user")
				// 550 5.1.1 : Mailbox not found
				// 550 Unknown address error ‘MAILBOX NOT FOUND’
				|| err_string.contains("not found")
				// 550 5.1.1 : Invalid mailbox
				|| err_string.contains("invalid mailbox")
				// 550 5.1.1 Sorry, no mailbox here by that name
				|| err_string.contains("no mailbox")
				// 5.2.0 No such mailbox
				|| err_string.contains("no such mailbox")
				// 550 Requested action not taken: mailbox unavailable
				|| err_string.contains("mailbox unavailable")
				// 550 5.1.1 Is not a valid mailbox
				|| err_string.contains("not a valid mailbox")
				// No such recipient here
				|| err_string.contains("no such recipient")
				// 554 delivery error: This user doesn’t have an account
				|| err_string.contains("have an account")
			{
				return Ok(Deliverability {
					has_full_inbox: false,
					is_deliverable: false,
					is_disabled: false,
				});
			}

			Err(SmtpError::SmtpError(err))
		}
	}
}

/// Verify the existence of a catch-all on the domain.
async fn smtp_is_catch_all(
	smtp_client: &mut SmtpTransport,
	domain: &str,
) -> Result<bool, SmtpError> {
	// Create a random 15-char alphanumerical string.
	let mut rng = SmallRng::from_entropy();
	let random_email: String = iter::repeat(())
		.map(|()| rng.sample(Alphanumeric))
		.map(char::from)
		.take(15)
		.collect();
	let random_email = EmailAddress::new(format!("{}@{}", random_email, domain));

	email_deliverable(
		smtp_client,
		&random_email.expect("Email is correctly constructed. qed."),
	)
	.await
	.map(|deliverability| deliverability.is_deliverable)
}

async fn create_smtp_future(
	to_email: &EmailAddress,
	host: &Name,
	port: u16,
	domain: &str,
	input: &CheckEmailInput,
) -> Result<(bool, Deliverability), SmtpError> {
	// FIXME If the SMTP is not connectable, we should actually return an
	// Ok(SmtpDetails { can_connect_smtp: false, ... }).
	let mut smtp_client = connect_to_host(host, port, input).await?;

	let is_catch_all = smtp_is_catch_all(&mut smtp_client, domain)
		.await
		.unwrap_or(false);
	let deliverability = if is_catch_all {
		Deliverability {
			has_full_inbox: false,
			is_deliverable: true,
			is_disabled: false,
		}
	} else {
		let mut result = email_deliverable(&mut smtp_client, to_email).await;

		// Some SMTP servers automatically close the connection after an error,
		// so we should reconnect to perform a next command.
		//
		// Unfortunately `smtp_client.is_connected()` doesn't report about this,
		// so we can only check for "io: incomplete" SMTP error being returned.
		// https://github.com/async-email/async-smtp/issues/37
		if is_io_incomplete_smtp_error(&result) {
			let _ = smtp_client.close().await;
			smtp_client = connect_to_host(host, port, input).await?;
			result = email_deliverable(&mut smtp_client, to_email).await;
		}

		result?
	};

	smtp_client.close().await?;

	Ok((is_catch_all, deliverability))
}

/// Indicates whether the given [`Result`] represents an `io: incomplete`
/// [`SmtpError::SmtpError`].
fn is_io_incomplete_smtp_error<T>(result: &Result<T, SmtpError>) -> bool {
	if let Err(SmtpError::SmtpError(AsyncSmtpError::Io(err))) = result {
		err.to_string().as_str() == "incomplete"
	} else {
		false
	}
}

/// Get all email details we can from one single `EmailAddress`.
pub async fn check_smtp(
	to_email: &EmailAddress,
	host: &Name,
	port: u16,
	domain: &str,
	input: &CheckEmailInput,
) -> Result<SmtpDetails, SmtpError> {
	// FIXME Is this `contains` too lenient?
	if input.yahoo_use_api && domain.to_lowercase().contains("yahoo") {
		return yahoo::check_yahoo(to_email, input)
			.await
			.map_err(|err| err.into());
	}

	let fut = create_smtp_future(to_email, host, port, domain, input);
	let (is_catch_all, deliverability) = if let Some(smtp_timeout) = input.smtp_timeout {
		future::timeout(smtp_timeout, fut).await??
	} else {
		fut.await?
	};

	Ok(SmtpDetails {
		can_connect_smtp: true,
		has_full_inbox: deliverability.has_full_inbox,
		is_catch_all,
		is_deliverable: deliverability.is_deliverable,
		is_disabled: deliverability.is_disabled,
	})
}

#[cfg(test)]
mod tests {
	use super::{check_smtp, CheckEmailInput, SmtpError};
	use async_smtp::EmailAddress;
	use std::{str::FromStr, time::Duration};
	use tokio::runtime::Runtime;
	use trust_dns_proto::rr::Name;

	#[test]
	fn should_timeout() {
		let runtime = Runtime::new().unwrap();

		let to_email = EmailAddress::from_str("foo@gmail.com").unwrap();
		let host = Name::from_str("gmail.com").unwrap();
		let mut input = CheckEmailInput::default();
		input.set_smtp_timeout(Duration::from_millis(1));

		let res = runtime.block_on(check_smtp(&to_email, &host, 25, "gmail.com", &input));
		match res {
			Err(SmtpError::TimeoutError(_)) => (),
			_ => panic!("check_smtp did not time out"),
		}
	}
}
