use cli::Args;
use std;

pub fn run(args: Args) {
	if let Args::Vendor { source } = args {
		println!("#include <bits/stdc++.h>");
		std::process::Command::new("g++")
			.args(&["-I", "/usr/share/ci/dummy-includes", "-E", source.as_path().to_str().unwrap()])
			.stdout(std::process::Stdio::inherit())
			.spawn().unwrap()
			.wait().unwrap();
	}
}
