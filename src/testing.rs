use checkers::Checker;
use colored::*;
use error::*;
use std::{self, path::Path};
use strres::{exec, StrRes};
use util::timefn;

#[derive(PartialEq, Eq)]
pub enum TestResult {
	Accept,
	WrongAnswer,
	RuntimeError,
	IgnoredNoOut,
}
impl TestResult {
	pub fn format_long(&self) -> ColoredString {
		self.apply_color(match *self {
			TestResult::Accept => "ACCEPT       ",
			TestResult::WrongAnswer => "WRONG ANSWER ",
			TestResult::RuntimeError => "RUNTIME ERROR",
			TestResult::IgnoredNoOut => "IGNORED      ",
		})
	}

	pub fn apply_color(&self, s: &str) -> ColoredString {
		match *self {
			TestResult::Accept => s.green().bold(),
			TestResult::WrongAnswer => s.red().bold(),
			TestResult::RuntimeError => s.red().bold(),
			TestResult::IgnoredNoOut => s.yellow().bold(),
		}
	}
}
pub fn test_single(executable: &Path, input: StrRes, perfect_output: StrRes, checker: &Checker) -> R<(TestResult, std::time::Duration)> {
	let (my_output, timing) = timefn(|| exec(executable, input.clone()));
	Ok((
		if let Ok(output) = my_output {
			if checker.check(input, output, perfect_output)? {
				TestResult::Accept
			} else {
				TestResult::WrongAnswer
			}
		} else {
			TestResult::RuntimeError
		},
		timing,
	))
}
