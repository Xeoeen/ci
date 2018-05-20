use colored::*;
use std;
use std::io::Write;
use std::path::Path;
use error::*;
use std::process::Command;

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