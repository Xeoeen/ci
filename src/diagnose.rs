use colored::*;
use std;
use std::io::Write;
use std::path::Path;
use error::*;
use std::process::Command;

pub fn diagnose_app(app: &Path) -> Result<()>{
	let meta_app = std::fs::metadata(app).context(format_err!("Failed to execute {:?}", app))?;
	if app.extension().map(|ext| ext == "e").unwrap_or(false) && app.with_extension("cpp").exists() {
		let srcfile = app.with_extension("cpp");
		let meta_src = std::fs::metadata(srcfile).context("Could not extract metadata from source file")?;
		if meta_src.modified()? > meta_app.modified()? {
			eprint!("{} .e is older than corresponding .cpp file ({}). Continue?", "WARNING".red().bold(), app.display());
			std::io::stderr().flush()?;
			std::io::stdin().read_line(&mut String::new())?;
		}
	}
	Command::new(app).spawn().context(format_err!("Failed to execute {:?}", app))?.kill()?;
	Ok(())
}
