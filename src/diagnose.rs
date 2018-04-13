use colored::*;
use std;
use std::io::Write;
use std::path::Path;

pub fn diagnose_app(app: &Path) {
	if has_extension(app, "e") {
		let srcfile = app.with_extension("cpp");
		if srcfile.exists() && older_than(app, &srcfile) {
			warn(&format!(".e is older than corresponding .cpp file ({})", app.display()));
		}
	}
}

fn has_extension(path: &Path, ext: &str) -> bool {
	path.extension().map(|e| e == ext).unwrap_or(false)
}

fn warn(s: &str) {
	eprint!("{} {}. Continue? ", "WARNING".red().bold(), s);
	std::io::stderr().flush().unwrap();
	std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn older_than(a: &Path, b: &Path) -> bool {
	let meta1 = std::fs::metadata(a).unwrap();
	let meta2 = std::fs::metadata(b).unwrap();
	meta1.modified().unwrap() < meta2.modified().unwrap()
}