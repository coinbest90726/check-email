// check_if_email_exists
// Copyright (C) 2018-2019 Amaury Martiny

// check_if_email_exists is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// check_if_email_exists is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with check_if_email_exists.  If not, see <http://www.gnu.org/licenses/>.

extern crate lettre;
#[macro_use]
extern crate log;
extern crate native_tls;
extern crate rayon;
extern crate trust_dns_resolver;

use lettre::smtp::{SMTP_PORT, SUBMISSIONS_PORT, SUBMISSION_PORT};
use lettre::EmailAddress;
use rayon::prelude::*;

mod mx_hosts;
mod smtp;

/// Errors that are returned by email_exists
#[derive(Debug)]
pub enum EmailExistsError {
	BlockedByIsp,
}

pub fn email_exists(
	from_email: &EmailAddress,
	to_email: &EmailAddress,
) -> Result<bool, EmailExistsError> {
	debug!("Checking email '{}'", to_email);

	let domain = to_email.to_string();
	let domain = domain
		.as_str()
		.split("@")
		.skip(1)
		.next()
		.expect("We checked above that email is valid. qed.");
	debug!("Domain name is '{}'", domain);

	debug!("Getting MX lookup...");
	let hosts = mx_hosts::get_mx_lookup(domain);
	debug!("Found the following MX hosts {:?}", hosts);
	let ports = vec![SMTP_PORT, SUBMISSION_PORT, SUBMISSIONS_PORT]; // [25, 587, 465]
	let mut combinations = Vec::new(); // `(host, port)` combination
	for port in ports.into_iter() {
		for host in hosts.iter() {
			combinations.push((host.exchange(), port))
		}
	}

	combinations
		// Parallely find any combination that returns true for email_exists
		.par_iter()
		.flat_map(|(host, port)| smtp::email_exists_on_host(from_email, to_email, host, *port))
		.find_any(|_| true)
		// If all smtp calls timed out/got refused/errored, we assume that the
		// ISP is blocking relevant ports
		.ok_or(EmailExistsError::BlockedByIsp)
}
