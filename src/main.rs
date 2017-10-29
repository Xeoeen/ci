extern crate clap;

use std::path::{Path};

fn compile_cpp(source: &Path, output: &Path) {
    let mut kid = std::process::Command::new("c++")
        .args(&["-std=c++11", "-Wall", "-Wextra", "-g", "-D_GLIBCXX_DEBUG", source.to_str().unwrap(), "-o", output.to_str().unwrap()])
        .stderr(std::process::Stdio::inherit())
        .spawn().unwrap();
    kid.wait().unwrap();
}

fn run_build(args: &clap::ArgMatches) {
    let source_name = Path::new(args.value_of("input").unwrap());
    assert!(source_name.extension().unwrap() == "cpp");
    let exec_name = source_name.with_extension("e");
    compile_cpp(source_name, &exec_name);
}

fn main() {
    let args = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(clap::SubCommand::with_name("build")
            .arg(clap::Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("INPUT")
                .required(true)
                .index(1)))
        .get_matches();
    if let Some(subcmd_args) = args.subcommand_matches("build") {
        run_build(subcmd_args);
    }
}
