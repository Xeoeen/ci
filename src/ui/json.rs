use super::Ui;
use commands::list_resources::Resource;
use failure;
use serde_json;
use std::{
	io::{stdin, stdout, Write}, path::{Path, PathBuf}, time::Duration
};
use strres::StrRes;
use testing::TestResult;

pub struct Json;
impl Json {
	pub fn new() -> Json {
		Json
	}
}
impl Ui for Json {
	fn read_auth(&mut self, domain: &str) -> (String, String) {
		println!("{}", serde_json::to_string(&AuthRequest { domain: domain.to_owned() }).unwrap());
		stdout().flush().unwrap();
		let mut line = String::new();
		stdin().read_line(&mut line).unwrap();
		let resp: AuthResponse = serde_json::from_str(&line).unwrap();
		(resp.username, resp.password)
	}

	fn track_progress(&mut self, _verdict: &unijudge::Verdict, _finish: bool) {
		unimplemented!()
	}

	fn submit_success(&mut self, id: String) {
		println!("{}", serde_json::to_string(&SubmitResult { id }).unwrap());
	}

	fn test_list(&mut self, _paths: &[PathBuf]) {}

	fn print_resource_list(&mut self, resources: &[Resource]) {
		println!("{}", serde_json::to_string(&resources).unwrap());
	}

	fn print_resource(&mut self, data: &'_ [u8]) {
		println!("{}", serde_json::to_string(&data).unwrap()); // TODO base64 encode
	}

	fn print_test(&mut self, outcome: &TestResult, timing: Option<Duration>, in_path: &Path, output: Option<StrRes>) {
		println!(
			"{}",
			serde_json::to_string(&SingleTest {
				in_path,
				outcome,
				output: output.map(|o| o.get_string().unwrap()),
				timing
			})
			.unwrap()
		);
	}

	fn print_finish_test(&mut self, _success: bool) {}

	fn print_finish_init(&mut self) {}

	fn print_transpiled(&mut self, _compiled: &str) {
		unimplemented!()
	}

	fn print_found_test(&mut self, _test_str: &str) {
		unimplemented!()
	}

	fn print_error(&mut self, _error: failure::Error) {
		unimplemented!()
	}

	fn mt_generator_fail(&mut self, _i: i64) {
		unimplemented!()
	}

	fn mt_autogenerated(&mut self, _i: i64) {
		unimplemented!()
	}

	fn mt_good(&mut self, _t: Duration) {
		unimplemented!()
	}

	fn mt_bad(&mut self, _t: Duration) {
		unimplemented!()
	}

	fn mt_piece(&mut self, _result: &TestResult, _ti: Duration) {
		unimplemented!()
	}

	fn mt_piece_ignored(&mut self) {
		unimplemented!()
	}

	fn mt_piece_finish(&mut self) {
		unimplemented!()
	}

	fn warn(&mut self, _message: &str) {}

	fn notice(&mut self, _message: &str) {}
}

#[derive(Serialize)]
struct AuthRequest {
	domain: String,
}
#[derive(Deserialize)]
struct AuthResponse {
	username: String,
	password: String,
}
#[derive(Serialize)]
struct SingleTest<'a> {
	outcome: &'a TestResult,
	timing: Option<Duration>,
	in_path: &'a Path,
	output: Option<String>,
}

#[derive(Serialize)]
struct SubmitResult {
	id: String,
}
