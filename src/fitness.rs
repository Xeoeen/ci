use error::R;
use std::{self, io::Write, str::from_utf8};
use strres::StrRes;

pub trait Fitness {
	fn fitness(&self, input: StrRes) -> R<i64>;
}

struct Bytelen;
impl Fitness for Bytelen {
	fn fitness(&self, input: StrRes) -> R<i64> {
		Ok(-(input.get_string()?.len() as i64))
	}
}

struct App {
	app: String,
}
impl App {
	fn new(app: String) -> R<App> {
		// TODO: diagnose fitness
		Ok(App { app })
	}
}
impl Fitness for App {
	fn fitness(&self, input: StrRes) -> R<i64> {
		let mut kid = std::process::Command::new(&self.app)
			.stdin(std::process::Stdio::piped())
			.stdout(std::process::Stdio::piped())
			.spawn()?;
		{
			let stdin = kid.stdin.as_mut().unwrap();
			stdin.write_all(input.get_string()?.as_bytes())?;
		}
		let std::process::Output { status, stdout, .. } = kid.wait_with_output()?;
		if !status.success() {
			return Err(format_err!("fitness run failed"));
		}
		Ok(from_utf8(&stdout)?.trim().parse()?)
	}
}

pub fn parse_fitness(s: &str) -> R<Box<Fitness>> {
	match s {
		"@bytelen" => Ok(Box::new(Bytelen)),
		path => Ok(Box::new(App::new(path.to_owned())?)),
	}
}
