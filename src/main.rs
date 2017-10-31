#![feature(io)]

extern crate clap;
extern crate walkdir;
extern crate colored;
extern crate tempfile;

use std::path::{Path};
use std::io::{Read, Write};
use colored::*;

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

fn equal_bew(a: &str, b: &str) -> bool {
    let mut i = a.chars().peekable();
    let mut j = b.chars().peekable();
    while i.peek().is_some() && j.peek().is_some() {
        if i.peek().unwrap().is_whitespace() && j.peek().unwrap().is_whitespace() {
            while i.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
                i.next();
            }
            while j.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
                j.next();
            }
        } else {
            if i.peek() != j.peek() {
                return false;
            }
            i.next();
            j.next();
        }
    }
    return i.peek().is_none() && j.peek().is_none();
}

fn run_test<F: Fn(&Path, &Path, &str) -> bool>(args: &clap::ArgMatches, f: F) {
    let exec = args.value_of("exec").unwrap();
    let testdir = args.value_of("testdir").unwrap();
    let print_success = !args.is_present("no-print-success");
    for entry in walkdir::WalkDir::new(testdir).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        if let Some(ext) = entry.path().extension() {
            if ext != "in" { continue }
            let in_path = entry.path();
            let out_path = in_path.with_extension("out");

            let kid = std::process::Command::new(exec)
                .stdin(std::fs::File::open(in_path).unwrap())
                .stdout(std::process::Stdio::piped())
                .spawn().unwrap();
            let mut output_kid = String::new();
            kid.stdout.unwrap().read_to_string(&mut output_kid).unwrap();

            let correct = f(in_path, &out_path, &output_kid);

            if correct {
                if print_success {
                    println!("{} {}", "TEST RUN SUCCESS".green().bold(), in_path.display());
                }
            } else {
                println!("{} {}", "TEST RUN FAILURE".red().bold(), in_path.display());
            }
        }
    }
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
        .subcommand(clap::SubCommand::with_name("test")
            .arg(clap::Arg::with_name("exec")
                .value_name("EXEC")
                .required(true)
                .index(1))
            .arg(clap::Arg::with_name("testdir")
                .value_name("TESTDIR")
                .required(true)
                .index(2))
            .arg(clap::Arg::with_name("checker")
                .long("checker")
                .takes_value(true))
            .arg(clap::Arg::with_name("no-print-success")
                .long("no-print-success")))
        .get_matches();
    if let Some(subcmd_args) = args.subcommand_matches("build") {
        run_build(subcmd_args);
    } else if let Some(subcmd_args) = args.subcommand_matches("test") {
        if let Some(checker) = subcmd_args.value_of("checker") {
            run_test(subcmd_args, |in_path, out_path, mine_str| -> bool {
                let mut mine_file = tempfile::NamedTempFile::new().unwrap();
                write!(mine_file, "{}", mine_str).unwrap();
                let mine_path = mine_file.path();
                std::process::Command::new(checker)
                    .args(&[in_path, mine_path, out_path])
                    .status().unwrap()
                    .success()
            });
        } else {
            run_test(subcmd_args, |_in_path, out_path, mine_str| -> bool {
                let mut out_file = std::fs::File::open(out_path).unwrap();
                let mut out_str = String::new();
                out_file.read_to_string(&mut out_str).unwrap();
                equal_bew(mine_str, &out_str)
            });
        }
    }
}
