use super::Ui;
use chrono::Local;
use colored::{ColoredString, Colorize};
use commands::{self, tracksubmit::Status};
use pbr;
use rpassword;
use std::{
	io::{stderr, stdin, Stderr, Write}, path::Path, time::Duration
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
		match status {
			Status::InitialPending => eprintln!("{} {}", Local::now(), "Initial tests pending...".white().bold()),
			// 			Status::RevealPending { examples } => eprintln!("{} {} {}", Local::now(), self.format_track_examples(examples), "Reveal pending...".white().bold()),
			// 			Status::RevealReady { examples } => eprintln!("{} {} {}", Local::now(), self.format_track_examples(examples), "Reveal ready.".white().bold()),
			Status::ScorePending { examples } => eprintln!("{} {} {}", Local::now(), self.format_track_examples(examples), "Score pending...".white().bold()),
			Status::ScoreReady { examples, score } => eprintln!("{} {} {}", Local::now(), self.format_track_examples(examples), self.format_score(*score)),
		}
	}
}

impl Human {
	fn format_track_examples(&self, examples: &commands::tracksubmit::Examples) -> String {
		format!(
			"Initial tests {}.",
			match examples {
				commands::tracksubmit::Examples::OK => "passed".green().bold(),
				commands::tracksubmit::Examples::Wrong => "failed".red().bold(),
			}
		)
	}

	fn format_score(&self, score: i64) -> ColoredString {
		if score == 0 {
			score.to_string().red().bold()
		} else if score == 100 {
			score.to_string().green().bold()
		} else {
			score.to_string().blue().bold()
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
