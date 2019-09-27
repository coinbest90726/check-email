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

/// E2E tests

#[cfg(test)]
mod tests {
	use check_if_email_exists_core::email_exists;

	#[test]
	fn should_output_error_for_invalid_email() {
		assert_eq!(
			serde_json::to_string(&email_exists("foo", "user@example.org")).unwrap(),
			"{\"mx\":{\"error\":{\"type\":\"Skipped\"}},\"smtp\":{\"error\":{\"type\":\"Skipped\"}},\"syntax\":{\"error\":{\"type\":\"SyntaxError\",\"message\":\"invalid email address\"}}}"
		);
	}

	#[test]
	fn should_output_error_for_invalid_mx() {
		assert_eq!(
			serde_json::to_string(&email_exists("foo@bar.baz", "user@example.org")).unwrap(),
			"{\"mx\":{\"error\":{\"type\":\"ResolveError\",\"message\":\"no record found for name: bar.baz type: MX class: IN\"}},\"smtp\":{\"error\":{\"type\":\"Skipped\"}},\"syntax\":{\"address\":\"foo@bar.baz\",\"domain\":\"bar.baz\",\"username\":\"foo\",\"valid_format\":true}}"
		);
	}

	#[test]
	#[ignore]
	fn should_output_error_when_blocked_by_isp() {
		assert_eq!(
			serde_json::to_string(&email_exists("someone@fastmail.com", "user@example.org")).unwrap(),
			"{\"mx\":{\"records\":[\"in1-smtp.messagingengine.com.\",\"in2-smtp.messagingengine.com.\"]},\"smtp\":{\"error\":{\"type\":\"BlockedByIsp\"}},\"syntax\":{\"address\":\"someone@fastmail.com\",\"domain\":\"fastmail.com\",\"username\":\"someone\",\"valid_format\":true}}"
		);
	}
}
