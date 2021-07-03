# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### [0.8.24](https://github.com/amaurymartiny/check-if-email-exists/compare/v0.8.23...v0.8.24) (2021-07-03)


### Features

* Add `CheckEmailInput` setter `set_` prefix to differentiate with accessing fields ([#933](https://github.com/amaurymartiny/check-if-email-exists/issues/933)) ([276f656](https://github.com/amaurymartiny/check-if-email-exists/commit/276f6561e7a98af6415dbd4645d84cbe697b738e))
* Add deprecated warning when running HTTP server ([#943](https://github.com/amaurymartiny/check-if-email-exists/issues/943)) ([e4b1570](https://github.com/amaurymartiny/check-if-email-exists/commit/e4b1570a8be5573f7394a3139f34ab021452cc3a))

### [0.8.23](https://github.com/amaurymartiny/check-if-email-exists/compare/v0.8.22...v0.8.23) (2021-06-20)


### Bug Fixes

* Add serde (De)Serialize to pub structs ([#931](https://github.com/amaurymartiny/check-if-email-exists/issues/931)) ([949475d](https://github.com/amaurymartiny/check-if-email-exists/commit/949475dee4a1ed96e873688e7432c702eb30af62))

### [0.8.22](https://github.com/amaurymartiny/check-if-email-exists/compare/v0.8.21...v0.8.22) (2021-03-31)


This is an empty release just to re-run the CI process for building binaries.

### [0.8.21](https://github.com/amaurymartiny/check-if-email-exists/compare/v0.8.20...v0.8.21) (2021-03-31)


This is an empty release just to re-run the CI process for building binaries.

### [0.8.20](https://github.com/amaurymartiny/check-if-email-exists/compare/v0.8.19...v0.8.20) (2021-03-30)


### Updates

* This release only bumps versions of dependencies, and does not introduce any bugfix or improvements.

### [0.8.19](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.18...v0.8.19) (2021-01-10)


### Features

* Consider CLI config parameters in HTTP request checks ([#827](https://github.com/reacherhq/check-if-email-exists/issues/827)) ([88b751a](https://github.com/reacherhq/check-if-email-exists/commit/88b751a17f4367c990e8a54661e3872898afd10f))


### Bug Fixes

* Reconnect auto-closed SMTP connections by foreign server ([#825](https://github.com/reacherhq/check-if-email-exists/issues/825)) ([01ccf0d](https://github.com/reacherhq/check-if-email-exists/commit/01ccf0d2363475d486bb9827e3e3b9d6954bc032))

### [0.8.18](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.17...v0.8.18) (2021-01-07)


### Bug Fixes

* Check deliverability using successful response code instead of message parsing ([#822](https://github.com/reacherhq/check-if-email-exists/issues/822)) ([39d0ecd](https://github.com/reacherhq/check-if-email-exists/commit/39d0ecdeaf078dce5cdb59cba95ab9e02bce11ee))

### [0.8.17](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.16...v0.8.17) (2021-01-05)


### Bug Fixes

* Add better checks for existing mailboxes ([#819](https://github.com/reacherhq/check-if-email-exists/issues/819)) ([9f88d01](https://github.com/reacherhq/check-if-email-exists/commit/9f88d01fad2c8de898aa35645bab95a14a147393))

### [0.8.16](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.15...v0.8.16) (2020-12-07)


### Features

* Add proxy_host and proxy_port info to HTTP ([#770](https://github.com/reacherhq/check-if-email-exists/issues/770)) ([123f431](https://github.com/reacherhq/check-if-email-exists/commit/123f431e10e90339e030783582d6e4c4919c1a33))

### [0.8.15](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.14...v0.8.15) (2020-11-11)


### Bug Fixes

* Don't check inputted email if catch-all ([#714](https://github.com/reacherhq/check-if-email-exists/issues/714)) ([5129dd1](https://github.com/reacherhq/check-if-email-exists/commit/5129dd1374d3ef93db632f6d7e140e3ce69369b2))
* Fix 'reached the type-length limit while instantiating' ([#665](https://github.com/reacherhq/check-if-email-exists/issues/665)) ([fa040fd](https://github.com/reacherhq/check-if-email-exists/commit/fa040fda8b16ca4d540829ee72f9d7b07ef77fdd))

### [0.8.14](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.13...v0.8.14) (2020-09-24)


### Features

* Add optional timeout on smtp verification ([#611](https://github.com/reacherhq/check-if-email-exists/issues/611)) ([c70de7d](https://github.com/reacherhq/check-if-email-exists/commit/c70de7dcac1811596c78e14888a4258e9db408ed))


### Bug Fixes

* Add more known errors for invalid email ([#543](https://github.com/reacherhq/check-if-email-exists/issues/543)) ([ad209c7](https://github.com/reacherhq/check-if-email-exists/commit/ad209c7ecb3f5aa466f31e293a05734b5edf5f6a))

### [0.8.13](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.12...v0.8.13) (2020-08-04)


### Bug Fixes

* **ci:** Put lto flag in cargo.toml ([#531](https://github.com/reacherhq/check-if-email-exists/issues/531)) ([00cbc1f](https://github.com/reacherhq/check-if-email-exists/commit/00cbc1fd46743c7579809a09b3897259213af496))

### [0.8.12](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.11...v0.8.12) (2020-08-04)


### Bug Fixes

* Add "recipient address accepted" check ([#489](https://github.com/reacherhq/check-if-email-exists/issues/489)) ([5d1e72a](https://github.com/reacherhq/check-if-email-exists/commit/5d1e72ae165f335ab97a96c3806e3293289187a2))
* http request body to use `to_emails` ([#502](https://github.com/reacherhq/check-if-email-exists/issues/502)) ([36aed56](https://github.com/reacherhq/check-if-email-exists/commit/36aed567cf705ef8d20489b2275e3d8ba58b75bb))

### [0.8.11](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.10...v0.8.11) (2020-07-11)


### Bug Fixes

* Add "Invalid email address" check ([#471](https://github.com/reacherhq/check-if-email-exists/issues/471)) ([3b03617](https://github.com/reacherhq/check-if-email-exists/commit/3b03617b81a1f9f6bc1bc6edc8c5d6d9f87eabbb))
* Add possibility to use proxy in Yahoo API request ([#472](https://github.com/reacherhq/check-if-email-exists/issues/472)) ([aafcedf](https://github.com/reacherhq/check-if-email-exists/commit/aafcedf9b9135a6550e7aa2da5d7ca5898da9b53))

### [0.8.10](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.9...v0.8.10) (2020-07-04)


### Bug Fixes

* Correct message parsing for "receiving at a rate" error ([#462](https://github.com/reacherhq/check-if-email-exists/issues/462)) ([4b31706](https://github.com/reacherhq/check-if-email-exists/commit/4b31706228a6e81852505ec21a0f70d5472b1385))

### [0.8.9](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.8...v0.8.9) (2020-07-04)


### Features

* Make using Yahoo API optional ([#460](https://github.com/reacherhq/check-if-email-exists/issues/460)) ([1e42f0a](https://github.com/reacherhq/check-if-email-exists/commit/1e42f0abef27dcea9a467f677ef9a080a3cc0f18))

### [0.8.8](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.7...v0.8.8) (2020-06-28)


### Bug Fixes

* Add debug logs for Yahoo ([e534670](https://github.com/reacherhq/check-if-email-exists/commit/e53467006f9fa435993ea58b1753ce5baa059d2a))

### [0.8.7](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.6...v0.8.7) (2020-06-28)


### Bug Fixes

* Add "recipient unknown" check ([#446](https://github.com/reacherhq/check-if-email-exists/issues/446)) ([deca071](https://github.com/reacherhq/check-if-email-exists/commit/deca071583e34bb9c5836d5238dd51975f827cdc))

### [0.8.6](https://github.com/reacherhq/check-if-email-exists/compare/v0.8.5...v0.8.6) (2020-06-28)


### Bug Fixes

* Add additional error check for undeliverable ([#374](https://github.com/reacherhq/check-if-email-exists/issues/374)) ([e52a8f0](https://github.com/reacherhq/check-if-email-exists/commit/e52a8f0941fd53c9b087e6e59a7018d11af72dff))
* Use HTTP requests to verify Yahoo emails ([#412](https://github.com/reacherhq/check-if-email-exists/issues/412)) ([5fad57d](https://github.com/reacherhq/check-if-email-exists/commit/5fad57d88ef92d65c7d493cdcb45eff347d6a286))

### [0.8.5](https://github.com/reacherhq/check_if_email_exists/compare/v0.8.4...v0.8.5) (2020-05-21)


### Features

* Expose misc, syntax, mx, smtp modules ([#373](https://github.com/reacherhq/check_if_email_exists/issues/373)) ([7c1d741](https://github.com/reacherhq/check_if_email_exists/commit/7c1d741f00b3a807b190140a1d91a42bce29470c))

### [0.8.4](https://github.com/reacherhq/check_if_email_exists/compare/v0.8.3...v0.8.4) (2020-05-19)


### Bug Fixes

* Fix is_reachable with wrong syntax ([#352](https://github.com/reacherhq/check_if_email_exists/issues/352)) ([b0f0209](https://github.com/reacherhq/check_if_email_exists/commit/b0f02090edc0bb8947ab826415fa3bf8b5db55f0))

### [0.8.3](https://github.com/reacherhq/check_if_email_exists/compare/v0.8.2...v0.8.3) (2020-05-12)


### Bug Fixes

* Lowercase Reachable enum variants ([#351](https://github.com/reacherhq/check_if_email_exists/issues/351)) ([b88c20e](https://github.com/reacherhq/check_if_email_exists/commit/b88c20ef5bc947ecd8cc646a9e6c583df0bef4d7))

### [0.8.2](https://github.com/reacherhq/check_if_email_exists/compare/v0.8.1...v0.8.2) (2020-05-12)


### Features

* Add `is_reachable` top field ([#350](https://github.com/reacherhq/check_if_email_exists/issues/350)) ([e7abb17](https://github.com/reacherhq/check_if_email_exists/commit/e7abb17ef29610fbe9210f42830c0ba02bb130b7))
* Detect role-based accounts ([#348](https://github.com/reacherhq/check_if_email_exists/issues/348)) ([7c612fd](https://github.com/reacherhq/check_if_email_exists/commit/7c612fda110729ece094d0b022db05fa4e6b27b4))


### Bug Fixes

* Add "Unknown user" smtp error check ([#347](https://github.com/reacherhq/check_if_email_exists/issues/347)) ([47eb578](https://github.com/reacherhq/check_if_email_exists/commit/47eb5780f692f54aadf264b107996bb2d1a31a56))
* Add more error strings matching ([#323](https://github.com/reacherhq/check_if_email_exists/issues/323)) ([f5392d4](https://github.com/reacherhq/check_if_email_exists/commit/f5392d4befcee6e4d935e1585066eae3b57ec6fa))

### [0.8.1](https://github.com/reacherhq/check_if_email_exists/compare/v0.8.0...v0.8.1) (2020-05-09)


### Bug Fixes

* Lowercase the error string before matching ([#321](https://github.com/reacherhq/check_if_email_exists/issues/321)) ([d983b2f](https://github.com/reacherhq/check_if_email_exists/commit/d983b2fe960ed46c4bd03c55b39d7ea58be5124f))

## [0.8.0](https://github.com/reacherhq/check_if_email_exists/compare/v0.7.1...v0.8.0) (2020-05-08)


### ⚠ BREAKING CHANGES

* This new version includes an overhaul of the codebase, mainly to prepare the groundwork for the upcoming work on bulk validation. These changes include:

- The main function `email_exists` has been renamed to `check_email`:

```diff
- email_exists(&input).await;
+ check_email(&input).await;
```

- The input `EmailInput` has been renamed to `CheckEmailInput`. Its `::new()` method, instead of taking a single `String`, now takes `Vec<String>`.

- The output `SingleEmail` has been renamed to `CheckEmailOutput`. The main function `check_emails` now returns a `Vec<CheckEmailOutput>`.

```rust
pub async fn check_email(inputs: &CheckEmailInput) -> Vec<CheckEmailOutput>
```

- The `syntax` field in `CheckEmailOutput` is no longer a `Result<SyntaxDetails, SyntaxError>`, but only `SyntaxDetails`. Error cases are guaranteed not to happen for syntax validation.

- The `misc`, `mx`, and `smtp` fields' signatures stay the same: `Result<{Misc,Mx,Smtp}Details, {Misc,Mx,Smtp}Error>`. However, the `Result` is an `Err` only when an internal error arrives. In case of errors due to user input (e.g. incorrect email inputted), the `Default` trait has been implemented on `{Misc,Mx,Smtp}Details` and will be returned. As such, the `Skipped` variant of error enums has been removed.

```diff
{
  "input": "foo@bar.baz",
  "mx": {
-    "error": { "cannot resolve" }
+    "accepts_mail": false, // This is Default
+    "records": [] // This is Default
  }
```

- The `misc`, `mx`, `smtp`, `syntax` modules have been made private.
* The field `syntax.valid_format` has been renamed to `syntax.is_valid_syntax`.

### Bug Fixes

* Rename valid_format to is_valid_syntax ([#288](https://github.com/reacherhq/check_if_email_exists/issues/288)) ([eae6482](https://github.com/reacherhq/check_if_email_exists/commit/eae64821c31d0193f77d9137ec4e7d6131f91ccb))


* Rename main function to `check_email` ([#319](https://github.com/reacherhq/check_if_email_exists/issues/319)) ([bd12b7d](https://github.com/reacherhq/check_if_email_exists/commit/bd12b7dbbd6c090fcdf80e3d6bbe475cd1d82b9a))

### [0.7.1](https://github.com/reacherhq/check_if_email_exists/compare/v0.7.0...v0.7.1) (2020-04-14)


### Features

* Add possibility to verify emails via proxy ([#286](https://github.com/reacherhq/check_if_email_exists/issues/286)) ([a0ab93f](https://github.com/reacherhq/check_if_email_exists/commit/a0ab93fde5105d594a8280b942d337ff76fbb517))

## [0.7.0](https://github.com/reacherhq/check_if_email_exists/compare/v0.6.7...v0.7.0) (2020-03-26)


### ⚠ BREAKING CHANGES

* `email_exists` only takes one input now, an `EmailInput` which is built using the builder pattern.
```diff
- use check_if_email_exists::email_exists;
+ use check_if_email_exists::{email_exists, EmailInput};

- email_exists("someone@gmail.com", "user@example.org");
+ email_exists(
+   EmailInput::new("someone@gmail.com".to_string()).from_email("user@example.org".to_string())
+ )
```

`EmailInput` additionally takes a `hello_name()` method, which is used to set the name in the EHLO smtp command.

`--from` in CLI has been replaced with `--from-email`.

### Features

* Use builder pattern for EmailInput ([#254](https://github.com/reacherhq/check_if_email_exists/issues/254)) ([0c85d36](https://github.com/reacherhq/check_if_email_exists/commit/0c85d36cdccb37d8da9566f7e7baf5dbbd266740))

### [0.6.7](https://github.com/reacherhq/check_if_email_exists/compare/v0.6.6...v0.6.7) (2020-03-20)

### [0.6.6](https://github.com/reacherhq/check_if_email_exists/compare/v0.6.1...v0.6.6) (2020-03-01)


### Bug Fixes

* Allow http to listen to $PORT env variable ([#215](https://github.com/reacherhq/check_if_email_exists/issues/215)) ([3b0c262](https://github.com/reacherhq/check_if_email_exists/commit/3b0c262763bc9d52855ced90aa2a435a97d35d8b))

### [0.6.1](https://github.com/reacherhq/check_if_email_exists/compare/v0.6.0...v0.6.1) (2020-02-18)


### Features

* Add --http-host flag to CLI ([#197](https://github.com/reacherhq/check_if_email_exists/issues/197)) ([55657b2](https://github.com/reacherhq/check_if_email_exists/commit/55657b251fcc22fad2ae53da4f62a017ff01d035))

## [0.6.0](https://github.com/reacherhq/check_if_email_exists/compare/v0.5.0...v0.6.0) (2019-12-01)


### ⚠ BREAKING CHANGES

* - The `is_disposable` subfield has been moved from the `mx` field to a separate `misc` field

### Features

* Add a HTTP server behind the `--http` flag ([#85](https://github.com/reacherhq/check_if_email_exists/issues/85)) ([d8b733e](https://github.com/reacherhq/check_if_email_exists/commit/d8b733e5a571c512644b34219b5f2dfd9dc717b3))
* Add Dockerfile & `x86_64-unknown-linux-musl` target ([#86](https://github.com/reacherhq/check_if_email_exists/issues/86)) ([cba1241](https://github.com/reacherhq/check_if_email_exists/commit/cba124110be04d7febfeab68a6b825197b3aa1fb))

# [0.5.0](https://github.com/reacherhq/check_if_email_exists/compare/v0.4.0...v0.5.0) (2019-11-16)


### Code Refactoring

* Use futures ([#78](https://github.com/reacherhq/check_if_email_exists/issues/78)) ([0e1f6b0](https://github.com/reacherhq/check_if_email_exists/commit/0e1f6b0))


### BREAKING CHANGES

* - The main function `email_exists` now returns a Future:
```rust
pub async fn email_exists(to_email: &str, from_email: &str) -> SingleEmail {}
```
- The `SmtpError::SmtpError` has been renamed to `SmtpError::LettreError` to show the underlying error more correctly (i.e., coming from `lettre` crate).
- The `BlockedByISP` error has been removed. Instead, you'll see e.g. `"connection refused"`, or whatever is returned by the SMTP server:
```json
{
  // ...,
  "smtp": {
    "error": {
      "type": "LettreError",
      "message": "connection refused"
    }
  },
}
```



# [0.4.0](https://github.com/reacherhq/check_if_email_exists/compare/v0.3.2...v0.4.0) (2019-09-30)


### Features

* Add disposable email check ([#64](https://github.com/reacherhq/check_if_email_exists/issues/64)) ([1b2cea3](https://github.com/reacherhq/check_if_email_exists/commit/1b2cea3))


### BREAKING CHANGES

* the `smtp`'s object keys have changed. Instead of
```
{
  "deliverable": ...,
  "full_inbox": ...,
  "has_catch_all": ...
}
```
it now returns 
```
{
  "has_full_inbox": ...,
  "is_deliverable": ...,
  "is_disabled": ...,
  "is_catch_all": ...
}
```
where `is_disabled` checks if the address has been disabled/blocked by the email provider



## [0.3.2](https://github.com/reacherhq/check_if_email_exists/compare/v0.3.1...v0.3.2) (2019-09-26)


### Bug Fixes

* **core:** SyntaxError also is type & message ([#60](https://github.com/reacherhq/check_if_email_exists/issues/60)) ([996633b](https://github.com/reacherhq/check_if_email_exists/commit/996633b))



## [0.3.1](https://github.com/reacherhq/check_if_email_exists/compare/v0.3.0...v0.3.1) (2019-09-26)


### Bug Fixes

* Don't use virtual workspace, be able to build ([#59](https://github.com/reacherhq/check_if_email_exists/issues/59)) ([6c93893](https://github.com/reacherhq/check_if_email_exists/commit/6c93893))



# [0.3.0](https://github.com/reacherhq/check_if_email_exists/compare/v0.2.3...v0.3.0) (2019-09-26)


### Features

* New error JSON format ([#56](https://github.com/reacherhq/check_if_email_exists/issues/56)) ([fec4315](https://github.com/reacherhq/check_if_email_exists/commit/fec4315))
* Output JSON information with CLI ([#53](https://github.com/reacherhq/check_if_email_exists/issues/53)) ([1d026d5](https://github.com/reacherhq/check_if_email_exists/commit/1d026d5))
* Return Result<EmailDetails> instead of Result<bool>, with much more details ([#23](https://github.com/reacherhq/check_if_email_exists/issues/23)) ([39b13f5](https://github.com/reacherhq/check_if_email_exists/commit/39b13f5))



## [0.2.3](https://github.com/reacherhq/check_if_email_exists/compare/v0.2.2...v0.2.3) (2019-05-09)


### Bug Fixes

* Update version to correct version in cli ([992777c](https://github.com/reacherhq/check_if_email_exists/commit/992777c))



## [0.2.2](https://github.com/reacherhq/check_if_email_exists/compare/v0.2.1...v0.2.2) (2019-05-09)


### Bug Fixes

* Fix travis and appveyor to build binaries ([f743e67](https://github.com/reacherhq/check_if_email_exists/commit/f743e67))



## [0.2.1](https://github.com/reacherhq/check_if_email_exists/compare/v0.2.0...v0.2.1) (2019-05-09)


### Bug Fixes

* Refactor app to make travis build binaries ([#17](https://github.com/reacherhq/check_if_email_exists/issues/17)) ([9616ef5](https://github.com/reacherhq/check_if_email_exists/commit/9616ef5))



# [0.2.0](https://github.com/reacherhq/check_if_email_exists/compare/v0.1.1...v0.2.0) (2019-05-09)


### Features

* Add serverless function ([#15](https://github.com/reacherhq/check_if_email_exists/issues/15)) ([532c4eb](https://github.com/reacherhq/check_if_email_exists/commit/532c4eb))
* Return Option<bool> instead of bool ([#13](https://github.com/reacherhq/check_if_email_exists/issues/13)) ([2aef345](https://github.com/reacherhq/check_if_email_exists/commit/2aef345))



## [0.1.1](https://github.com/reacherhq/check_if_email_exists/compare/v0.1.0...v0.1.1) (2018-12-29)


# 0.1.0 (2018-12-29)


### Features

* Change codebase to Rust ([#7](https://github.com/reacherhq/check_if_email_exists/pull/7)) ([05569e4](https://github.com/reacherhq/check_if_email_exists/commit/05569e4900b4467fa6d7f03086343fac753fe4ad))
