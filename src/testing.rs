use checkers::Checker;
use colored::{self, ColoredString, Colorize};
use error::*;
use std::{self, path::Path, time::Duration};
use strres::{exec, StrRes};
use util::timefn;

#[derive(PartialEq, Eq, Serialize)]
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
		s.color(match *self {
			TestResult::Accept => colored::Color::Green,
			TestResult::WrongAnswer => colored::Color::Red,
			TestResult::RuntimeError => colored::Color::Red,
			TestResult::IgnoredNoOut => colored::Color::Yellow,
		}).bold()
	}
}
pub fn test_single(executable: &Path, input: StrRes, perfect_output: StrRes, checker: &Checker, time_limit: Option<&Duration>) -> R<(StrRes, TestResult, std::time::Duration)> {
	let (my_output, timing) = timefn(|| exec(executable, input.clone(), time_limit));
	Ok((
		my_output.as_ref().map(|sr| sr.clone()).unwrap_or(StrRes::Empty),
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
