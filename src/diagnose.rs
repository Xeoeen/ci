use colored::*;
use std;
use std::io::Write;
use std::path::Path;

pub fn diagnose_app(app: &Path) {
	if app.extension().map(|ext| ext == "e").unwrap_or(false) && app.with_extension("cpp").exists() {
		let srcfile = app.with_extension("cpp");
		let meta_app = std::fs::metadata(app).unwrap();
		let meta_src = std::fs::metadata(srcfile).unwrap();
		if meta_src.modified().unwrap() > meta_app.modified().unwrap() {
			eprint!("{} .e is older than corresponding .cpp file ({}). Continue?", "WARNING".red().bold(), app.display());
			std::io::stderr().flush().unwrap();
			std::io::stdin().read_line(&mut String::new()).unwrap();
		}
	}
}
