use cli::Args;
use std;
use error::*;

pub fn run(args: Args) -> Result<()> {
	if let Args::Vendor { source } = args {
		println!("#include <bits/stdc++.h>");
		let status = std::process::Command::new("g++")
			.args(&["-I", "/usr/share/ci/dummy-includes", "-E", source.as_path().to_str().unwrap()])
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
	else {
		Err(Error::from(CliError::WrongCommand))
	}
}
