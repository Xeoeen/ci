use error::*;
use std;
use strres::StrRes;

pub trait Checker {
	fn check(&self, input: StrRes, my_output: StrRes, perfect_output: StrRes) -> R<bool>;
	fn is_default(&self) -> bool;
}

pub struct CheckerDiffOut;
impl Checker for CheckerDiffOut {
	fn check(&self, _input: StrRes, my_output: StrRes, perfect_output: StrRes) -> R<bool> {
		Ok(equal_bew(&my_output.get_string()?, &perfect_output.get_string()?))
	}

	fn is_default(&self) -> bool {
		true
	}
}

pub struct CheckerApp {
	app: String,
}

impl CheckerApp {
	pub fn new(app: String) -> R<CheckerApp> {
		// TODO: diagnose checker
		Ok(CheckerApp { app })
	}
}
impl Checker for CheckerApp {
	fn check(&self, input: StrRes, my_output: StrRes, perfect_output: StrRes) -> R<bool> {
		input.with_filename(|i_path| {
			my_output.with_filename(|mo_path| {
				perfect_output.with_filename(|po_path| {
					Ok(std::process::Command::new(&self.app)
						.args(&[i_path, mo_path, po_path])
						.status()
						.context(format_err!("Running checker {}", self.app))?
						.success())
				})
			})
		})
	}

	fn is_default(&self) -> bool {
		false
	}
}

fn equal_bew(a: &str, b: &str) -> bool {
	let mut i = a.chars().peekable();
	let mut j = b.chars().peekable();
	while i.peek().is_some() && j.peek().is_some() {
		if i.peek().unwrap().is_whitespace() && j.peek().unwrap().is_whitespace() {
			while i.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
				i.next();
			}
			while j.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
				j.next();
			}
		} else {
			if i.peek() != j.peek() {
				return false;
			}
			i.next();
			j.next();
		}
	}
	for c in i {
		if !c.is_whitespace() {
			return false;
		}
	}
	for c in j {
		if !c.is_whitespace() {
			return false;
		}
	}
	true
}
