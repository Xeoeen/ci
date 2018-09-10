use super::Ui;
use chrono::Local;
use colored::Colorize;
use commands::{
	list_resources::Resource, tracksubmit::{Compilation, Outcome, Status}
};
use pbr;
use rpassword;
use std::{
	io::{stderr, stdin, stdout, Stderr, Write}, path::Path, time::Duration
};
use strres::StrRes;
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

	fn track_progress(&self, status: &Status) {
		let message = match status {
			Status {
				compilation: Compilation::Pending,
				..
			} => "Compilation pending...".to_string(),
			Status {
				compilation: Compilation::Failure,
				..
			} => "Compilation error".to_string(),
			Status { initial: Outcome::Pending, .. } => "Initial tests pending...".to_string(),
			Status {
				full: Outcome::Pending, initial, ..
			} => format!("{}Score pending...", self.fmt_initial(initial)),
			Status { full, .. } => self.fmt_full(full),
		};
		eprintln!("{} {}", Local::now(), message);
	}

	fn submit_success(&self, id: String) {
		eprintln!("Solution submitted, submission id: {}", id);
	}

	fn print_resource_list(&self, resources: &[Resource]) {
		for resource in resources {
			println!("{}", resource.name.white().bold());
			println!("{}", resource.description);
			println!("{}", resource.filename);
			println!("{}", resource.id);
			println!("");
		}
	}

	fn print_resource(&self, data: &'_ [u8]) {
		stdout().write_all(&data).unwrap();
		stdout().flush().unwrap();
	}
}

impl Human {
	fn fmt_initial(&self, outcome: &Outcome) -> String {
		match outcome {
			Outcome::Score(score) => format!("Initial tests scored {}. ", score),
			Outcome::Failure => "Initial tests failed. ".to_string(),
			Outcome::Success => "Initial tests passed. ".to_string(),
			Outcome::Pending => panic!("formatting pending initiial tests"),
			Outcome::Unsupported => "".to_string(),
			Outcome::Skipped => "".to_string(),
			Outcome::Waiting => panic!("formatting waiting initial tests"),
		}
	}

	fn fmt_full(&self, outcome: &Outcome) -> String {
		match outcome {
			Outcome::Score(score) => format!("Scored {}!", score),
			Outcome::Failure => "Rejected".to_string(),
			Outcome::Success => "Accepted".to_string(),
			Outcome::Pending => panic!("formatting pending tests"),
			Outcome::Unsupported => panic!("formatting unsupported tests"),
			Outcome::Skipped => panic!("formatting skipped tests"),
			Outcome::Waiting => panic!("formatting waiting tests"),
		}
	}
}

impl ProgressBar for pbr::ProgressBar<Stderr> {
	fn print_test(&mut self, outcome: &TestResult, timing: Option<Duration>, in_path: &Path, output: Option<StrRes>) {
		let rstr = outcome.format_long();
		let timestr = timing.map(|timing| timefmt(timing)).unwrap_or("-.--s".to_owned()).blue().bold();
		pb_interwrite!(self, "{} {} {}", rstr, timestr, in_path.display());
		if let Some(output) = output {
			pb_interwrite!(self, "{}", output.get_string().unwrap());
		}
	}

	fn increment(&mut self) {
		self.inc();
	}
}
