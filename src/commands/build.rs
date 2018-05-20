use std;
use std::path::Path;
use std::ffi::OsStr;
use error::*;

pub enum CppVer {
	Cpp11,
	Cpp17,
}
impl CppVer {
	fn flag(&self) -> &'static str {
		match self {
			&CppVer::Cpp11 => "-std=c++11",
			&CppVer::Cpp17 => "-std=c++17",
		}
	}
}

fn compile_cpp(source: &Path, output: &Path, release: bool, cppver: CppVer) -> R<()> {
    let mut args = vec![];
	args.push(cppver.flag());
	args.extend_from_slice(&["-Wall", "-Wextra", "-Wconversion", "-Wno-sign-conversion"]);
    if release {
        args.push("-O2");
    } else {
        args.extend_from_slice(&["-g", "-D_GLIBCXX_DEBUG", "-fno-sanitize-recover=undefined"]);
    }
    args.push(source.to_str().unwrap());
    args.push("-o");
    args.push(output.to_str().unwrap());
    let mut kid = std::process::Command::new("clang++")
        .args(&args)
        .stderr(std::process::Stdio::inherit())
        .spawn()?;
	let status = kid.wait()?;

	if !status.success() {
		return Err(
			Error::from(
				E::NonZeroStatus(status.code().unwrap_or(101))
								.context(
									format_err!("Failed to compile using standard {} in {} mode",
										cppver.flag(), if release { "release" } else { "debug" } )
								)
			)
		);
	}
	Ok(())
}

pub fn run(source: &Path, release: bool, standard: CppVer) -> R<()> {
	ensure!(source.extension().unwrap_or(OsStr::new("")) == "cpp", E::InvalidFileExtension("cpp".to_string(), source.to_str().unwrap().to_string()));
	let executable = source.with_extension("e");
	compile_cpp(&source, &executable, release, standard)
}
