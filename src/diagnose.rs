use checkers::Checker;
use colored::Colorize;
use error::*;
use std::{self, io::Write, path::Path, process::Command};

pub fn diagnose_app(app: &Path) -> R<()> {
	std::fs::metadata(app).context(format_err!("Failed to execute {:?}", app))?;

	if has_extension(app, "e") {
		let srcfile = app.with_extension("cpp");
		if srcfile.exists() && older_than(app, &srcfile)? {
			warn(&format!(".e is older than corresponding .cpp file ({})", app.display()));
		}
	}

	Command::new(app).spawn().context(format_err!("Failed to execute {:?}", app))?.kill()?;
	Ok(())
}

pub fn diagnose_checker(checker: &Checker) -> R<()> {
	static TYPICAL_CHECKER_FILES: [&str; 3] = ["checker.cpp", "checker.e", "checker.py"];
	if checker.is_default() {
		for tcf in &TYPICAL_CHECKER_FILES {
			if Path::new(tcf).exists() {
				warn(&format!("No checker specified, but file {} present", tcf));
				break;
			}
		}
	}
	Ok(())
}

fn has_extension(path: &Path, ext: &str) -> bool {
	path.extension().map(|e| e == ext).unwrap_or(false)
}

fn warn(s: &str) {
	eprint!("{} {}. Continue? ", "WARNING".red().bold(), s);
	std::io::stderr().flush().unwrap();
	std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn older_than(a: &Path, b: &Path) -> R<bool> {
	let meta1 = std::fs::metadata(a)?;
	let meta2 = std::fs::metadata(b)?;
	Ok(meta1.modified()? < meta2.modified()?)
}
