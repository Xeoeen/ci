use super::Ui;
use serde_json;
use std::{
	io::{stdin, stdout, Write}, path::Path, time::Duration
};
use strres::StrRes;
use testing::TestResult;
use ui::ProgressBar;

pub struct Json;
impl Json {
	pub fn new() -> Json {
		Json
	}
}
impl Ui for Json {
	fn read_auth(&self, domain: &str) -> (String, String) {
		println!("{}", serde_json::to_string(&AuthRequest { domain: domain.to_owned() }).unwrap());
		stdout().flush().unwrap();
		let mut line = String::new();
		stdin().read_line(&mut line).unwrap();
		let resp: AuthResponse = serde_json::from_str(&line).unwrap();
		(resp.username, resp.password)
	}

	fn create_progress_bar(&self, _: usize) -> Box<ProgressBar> {
		Box::new(NoProgressBar)
	}
}

struct NoProgressBar;
impl ProgressBar for NoProgressBar {
	fn print_test(&mut self, outcome: &TestResult, timing: Option<Duration>, in_path: &Path, output: Option<StrRes>) {
		println!(
			"{}",
			serde_json::to_string(&SingleTest {
				outcome,
				timing,
				in_path,
				output: output.map(|sr| sr.get_string().unwrap())
			}).unwrap()
		);
	}

	fn increment(&mut self) {}
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
