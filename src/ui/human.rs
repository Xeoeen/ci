use super::Ui;
use pbr;
use rpassword;
use std::{
	borrow::Cow, io::{stderr, stdin, Stderr, Write}, path::Path, time::Duration
};
use strres::StrRes;
use term_painter::{Color::Blue, ToStyle};
use testing::TestResult;
use ui::{timefmt, ProgressBar};

pub struct Human;
impl Human {
	pub fn new() -> Human {
		Human
	}
}
impl Ui for Human {
	fn read_auth(&self, domain: &str) -> (String, String) {
		eprintln!("Login required to {}", domain);
		eprint!("  Username: ");
		stderr().flush().unwrap();
		let mut username = String::new();
		stdin().read_line(&mut username).unwrap();
		username = username.trim().to_owned();
		let password = rpassword::prompt_password_stderr("  Password: ").unwrap();
		(username, password)
	}

	fn create_progress_bar(&self, n: usize) -> Box<ProgressBar> {
		Box::new(pbr::ProgressBar::on(stderr(), n as u64))
	}
}

impl ProgressBar for pbr::ProgressBar<Stderr> {
	fn print_test(&mut self, outcome: &TestResult, timing: Option<Duration>, in_path: &Path, output: Option<StrRes>) {
		let rstr = outcome.format_long();
		let timestr = Blue.bold().paint(timing.map(|timing| Cow::Owned(timefmt(timing))).unwrap_or(Cow::Borrowed("-.--s")));
		pb_interwrite!(self, "{} {} {}", rstr, timestr, in_path.display());
		if let Some(output) = output {
			pb_interwrite!(self, "{}", output.get_string().unwrap());
		}
	}

	fn increment(&mut self) {
		self.inc();
	}
}
