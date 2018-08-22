macro_rules! pb_interwrite {
	($pb:expr, $fmt:expr $(,$arg:expr)*) => {
		{
			eprint!("\r\x1B[K");
			eprint!($fmt $(,$arg)*);
			eprintln!();
			$pb.tick();
		}
	};
}

macro_rules! eprint_flush {
	($fmt:expr $(,$arg:expr)*) => {
		{
			use std;
			use std::io::Write;
			eprint!($fmt $(,$arg)*);
			std::io::stderr().flush().unwrap();
		}
	};
}

mod human;
mod json;

use std;

pub fn timefmt(t: std::time::Duration) -> String {
	format!("{}.{:02}s", t.as_secs(), t.subsec_nanos() / 10_000_000)
}

pub trait Ui {
	fn read_auth(&self, domain: &str) -> (String, String);
	fn create_progress_bar(&self, n: usize) -> Box<ProgressBar>;
	fn track_progress(&self, status: &commands::tracksubmit::Status);
}

// TODO separate print_test and change this system into four traits: ProgressBar, BareUi, Ui: BareUi
pub trait ProgressBar {
	fn print_test(&mut self, outcome: &TestResult, timing: Option<Duration>, in_path: &Path, output: Option<StrRes>);
	fn increment(&mut self);
}

pub use self::{human::Human, json::Json};
use commands;
use std::{path::Path, time::Duration};
use strres::StrRes;
use testing::TestResult;
