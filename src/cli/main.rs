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

extern crate env_logger;
#[macro_use]
extern crate clap;
extern crate lettre;

use check_if_email_exists::email_exists;
use clap::App;
use std::process;

fn main() {
	env_logger::init();

	// The YAML file is found relative to the current file, similar to how modules are found
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();

	let from_email = matches.value_of("FROM_EMAIL").unwrap_or("user@example.org");
	let to_email = matches
		.value_of("TO_EMAIL")
		.expect("'TO_EMAIL' is required. qed.");

	match email_exists(&from_email, &to_email) {
		Ok(exists) => println!("{:?}", exists),
		Err(err) => {
			println!("{:?}", err);
			process::exit(1);
		}
	}
}
