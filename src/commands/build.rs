use super::super::*;

fn compile_cpp(source: &Path, output: &Path, release: bool, cppver: CppVer) {
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
        .spawn().unwrap();
    assert!(kid.wait().unwrap().success());
}

pub fn run(args: Args) {
	if let Args::Build { source, release, standard } = args {
		assert!(source.extension().unwrap() == "cpp");
		let executable = source.with_extension("e");
		compile_cpp(&source, &executable, release, standard);
	}
}
