use error::*;
use std;
use std::path::Path;

pub fn run(source: &Path) -> Result<()> {
	println!("#include <bits/stdc++.h>");
	let status = std::process::Command::new("g++")
		.args(&["-I", "/usr/share/ci/dummy-includes", "-E", source.to_str().unwrap()])
		.stdout(std::process::Stdio::inherit())
		.spawn()?
		.wait()?;
		if !status.success() {
			return Err(
				Error::from(
					RuntimeError::NonZeroStatus(status.code().unwrap_or(101))
								.context(format_err!("Failed to run preprocessor"))
				)
			);
		}
		Ok(())
}
