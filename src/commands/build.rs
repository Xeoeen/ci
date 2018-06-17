use error::*;
use std::{self, ffi::OsStr, path::Path};

pub enum CppVer {
	Cpp11,
	Cpp17,
}
impl CppVer {
	fn flag(&self) -> &'static str {
		match *self {
			CppVer::Cpp11 => "-std=c++11",
			CppVer::Cpp17 => "-std=c++17",
		}
	}
}

#[derive(Debug)]
pub enum Codegen {
	Debug,
	Release,
	Profile,
}

fn compile_cpp(source: &Path, output: &Path, codegen: &Codegen, cppver: &CppVer) -> R<()> {
	let mut args = vec![];
	args.push(cppver.flag());
	args.extend_from_slice(&["-Wall", "-Wextra", "-Wconversion", "-Wno-sign-conversion"]);
	args.extend_from_slice(match *codegen {
		Codegen::Debug => &["-g", "-D_GLIBCXX_DEBUG", "-fno-sanitize-recover=undefined"],
		Codegen::Release => &["-Ofast"],
		Codegen::Profile => &["-g", "-O2", "-fno-inline-functions"],
	});
	args.push(source.to_str().unwrap());
	args.push("-o");
	args.push(output.to_str().unwrap());
	let mut kid = std::process::Command::new("clang++").args(&args).stderr(std::process::Stdio::inherit()).spawn()?;
	let status = kid.wait()?;

	if !status.success() {
		return Err(Error::from(E::NonZeroStatus(status.code().unwrap_or(101)).context(format_err!(
			"Failed to compile using standard {} in {:?} mode",
			cppver.flag(),
			codegen
		))));
	}
	Ok(())
}

pub fn run(source: &Path, codegen: &Codegen, standard: &CppVer) -> R<()> {
	ensure!(
		source.extension().unwrap_or_else(|| OsStr::new("")) == "cpp",
		E::InvalidFileExtension("cpp".to_string(), source.to_str().unwrap().to_string())
	);
	let executable = source.with_extension("e");
	compile_cpp(&source, &executable, codegen, &standard)
}
