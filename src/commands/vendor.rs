use error::*;
use std::{self, path::Path};
use ui::Ui;

pub fn run(source: &Path, ui: &mut Ui) -> R<()> {
	let mut compiled = String::from("#include <bits/stdc++.h>\n");
	let op = std::process::Command::new("g++")
		.args(&["-I", "/usr/share/ci/dummy-includes", "-E", source.to_str().unwrap()])
		.stdout(std::process::Stdio::piped())
		.spawn()?
		.wait_with_output()?;
	if !op.status.success() {
		return Err(Error::from(
			E::NonZeroStatus(op.status.code().unwrap_or(101)).context(format_err!("Failed to run preprocessor")),
		));
	}
	compiled += std::str::from_utf8(&op.stdout)?;
	ui.print_transpiled(&compiled);
	Ok(())
}
