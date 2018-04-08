use std;
use strres::StrRes;
use error::*;

pub trait Checker {
	fn check(&self, input: StrRes, my_output: StrRes, perfect_output: StrRes) -> Result<bool>;
}

pub struct CheckerDiffOut;
impl Checker for CheckerDiffOut {
	fn check(&self, _input: StrRes, my_output: StrRes, perfect_output: StrRes) -> Result<bool> {
		Ok(equal_bew(&my_output.get_string()?, &perfect_output.get_string()?))
	}
}

pub struct CheckerApp {
	pub app: String,
}

impl Checker for CheckerApp {
	fn check(&self, input: StrRes, my_output: StrRes, perfect_output: StrRes) -> Result<bool> {
		input.with_filename(|i_path| {
			my_output.with_filename(|mo_path| {
				perfect_output.with_filename(|po_path| {
					Ok(std::process::Command::new(&self.app)
						.args(&[i_path, mo_path, po_path])
						.status().context(format_err!("Running checker {}", self.app))?
						.success())
				})
			})
		})
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
	while let Some(c) = i.next() {
		if !c.is_whitespace() {
			return false;
		}
	}
	while let Some(c) = j.next() {
		if !c.is_whitespace() {
			return false;
		}
	}
	true
}
