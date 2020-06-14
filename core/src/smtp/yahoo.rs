// check-if-email-exists
// Copyright (C) 2018-2020 Amaury Martiny

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

use super::SmtpDetails;
use crate::util::ser_with_display::ser_with_display;
use async_smtp::EmailAddress;
use http_types::Error as HttpError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::error::Error as SerdeError;
use std::fmt;

const SIGNUP_PAGE: &str = "https://login.yahoo.com/account/create?specId=yidReg&lang=en-US&src=&done=https%3A%2F%2Fwww.yahoo.com&display=login";
const SIGNUP_API: &str = "https://login.yahoo.com/account/module/create?validateField=yid";

/// The form inputs to pass into the HTTP request.
#[derive(Serialize)]
struct FormRequest {
	acrumb: String,
	#[serde(rename(serialize = "specId"))]
	spec_id: String,
	yid: String,
}

impl Default for FormRequest {
	fn default() -> Self {
		FormRequest {
			acrumb: "".into(),
			spec_id: "yidReg".into(),
			yid: "".into(),
		}
	}
}

impl FormRequest {
	fn new(acrumb: String, yid: String) -> Self {
		FormRequest {
			acrumb,
			yid,
			..Default::default()
		}
	}
}

/// One item in the response of the HTTP form request.
#[derive(Debug, Deserialize)]
struct FormResponseItem {
	error: String,
	name: String,
}

/// The response of the HTTP form request.
#[derive(Debug, Deserialize)]
struct FormResponse {
	errors: Vec<FormResponseItem>,
}

/// Possible errors when checking Yahoo email addresses.
#[derive(Debug, Serialize)]
pub enum YahooError {
	/// Error when sending an HTTP request. Used to verify Yahoo emails.
	#[serde(serialize_with = "ser_with_display")]
	HttpError(HttpError),
	/// Cannot find "acrumb" field in cookie.
	NoAcrumb,
	/// Cannot find cookie in Yahoo response.
	NoCookie,
	/// Error when serializing or deserializing HTTP requests and responses.
	#[serde(serialize_with = "ser_with_display")]
	SerdeError(SerdeError),
}

impl fmt::Display for YahooError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Customize so only `x` and `y` are denoted.
		write!(f, "{:?}", self)
	}
}

impl From<HttpError> for YahooError {
	fn from(error: HttpError) -> Self {
		YahooError::HttpError(error)
	}
}

impl From<SerdeError> for YahooError {
	fn from(error: SerdeError) -> Self {
		YahooError::SerdeError(error)
	}
}

/// Use well-crafted HTTP requests to verify if a Yahoo email address exists.
/// Inspired by https://github.com/hbattat/verifyEmail.
pub async fn check_yahoo(to_email: &EmailAddress) -> Result<SmtpDetails, YahooError> {
	let response = surf::get(SIGNUP_PAGE).await?;

	// Get the cookies from the response.
	let cookies = match response.header("Set-Cookie") {
		Some(x) => x,
		_ => {
			return Err(YahooError::NoCookie);
		}
	};

	let to_email = to_email.to_string();
	let username = to_email
		.split('@')
		.next()
		.expect("The email is well-formed. qed.");

	// From the cookies, fetch the "acrumb" field.
	let acrumb = match cookies.into_iter().next() {
		Some(x) => x.to_string(),
		_ => {
			return Err(YahooError::NoAcrumb);
		}
	};
	let re = Regex::new(r"s=(?P<acrumb>[^;]*)").expect("Correct regex. qed.");
	let acrumb = match re.captures(acrumb.as_ref()) {
		Some(x) => x,
		_ => {
			return Err(YahooError::NoAcrumb);
		}
	};

	// Mimic a real HTTP request.
	let response = surf::post(SIGNUP_API).set_header("Origin","https://login.yahoo.com")
    .set_header("X-Requested-With","XMLHttpRequest")
    .set_header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36")
    .set_header("Content-type", "application/x-www-form-urlencoded; charset=UTF-8")
    .set_header("Accept", "*/*")
    .set_header("Referer", SIGNUP_PAGE)
    .set_header("Accept-Encoding", "gzip, deflate, br")
    .set_header("Accept-Language", "en-US,en;q=0.8,ar;q=0.6")
    .set_header("Cookie", cookies)
    .body_json(&FormRequest::new(acrumb["acrumb"].to_string(), username.into()))?
    .recv_json::<FormResponse>()
    .await?;

	let username_exists = response
		.errors
		.iter()
		.any(|item| item.name == "yid" && item.error == "IDENTIFIER_EXISTS");

	Ok(SmtpDetails {
		can_connect_smtp: true,
		is_deliverable: username_exists,
		..Default::default()
	})
}
