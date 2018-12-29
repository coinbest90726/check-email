#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate lettre;
#[macro_use]
extern crate log;
extern crate native_tls;
extern crate rayon;
extern crate trust_dns_resolver;

use clap::App;
use lettre::smtp::{SMTP_PORT, SUBMISSIONS_PORT, SUBMISSION_PORT};
use rayon::prelude::*;
use std::process;

mod mx_hosts;
mod smtp;

fn main() {
	env_logger::init();

	// The YAML file is found relative to the current file, similar to how modules are found
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();

	let from_email = matches.value_of("from").unwrap_or("user@example.org");
	// Calling .unwrap() is safe here because "TO" is required
	let to_email = matches.value_of("TO").unwrap();

	debug!("User inputted email '{}'", to_email);

	let domain = match to_email.split("@").skip(1).next() {
		Some(i) => i,
		None => {
			error!("'{}' is not a valid email.", to_email);
			process::exit(1);
		}
	};
	debug!("Domain name is '{}'", domain);

	debug!("Getting MX lookup...");
	let hosts = mx_hosts::get_mx_lookup(domain);
	info!("Found the following MX hosts {:?}", hosts);
	let ports = vec![SMTP_PORT, SUBMISSION_PORT, SUBMISSIONS_PORT];
	let mut combinations = Vec::new(); // `(host, port)` combination
	for port in ports.into_iter() {
		for host in hosts.iter() {
			combinations.push((host.exchange(), port))
		}
	}

	let found = combinations
		.par_iter() // Parallelize the find_any
		.find_any(
			|(host, port)| match smtp::email_exists(from_email, to_email, host, *port) {
				Ok(val) => val,
				_ => false,
			},
		).is_some();

	println!("{}", found);
}
