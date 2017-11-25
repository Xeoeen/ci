#![feature(io)]

extern crate clap;
extern crate walkdir;
extern crate colored;
extern crate tempfile;
extern crate pbr;

mod strres;

use std::path::{Path};
use std::io::{Write};
use colored::*;
use strres::{StrRes, exec};

macro_rules! pb_interwrite {
	($pb:expr, $fmt:expr) => {
		std::io::stdout().write(format!("\r\x1B[K{}\n", $fmt).as_bytes()).unwrap();
		std::io::stdout().flush().unwrap();
		$pb.tick();
	};
	($pb:expr, $fmt:expr $(,$arg:expr)*) => {
		{
			let msg = format!($fmt $(,$arg)*);
			std::io::stdout().write(format!("\r\x1B[K{}\n", msg).as_bytes()).unwrap();
			std::io::stdout().flush().unwrap();
			$pb.tick();
		}
	};
}

fn compile_cpp(source: &Path, output: &Path, release: bool) {
    let mut args = vec!["-std=c++11", "-Wall", "-Wextra"];
    if release {
        args.push("-O2");
    } else {
        args.extend_from_slice(&["-g", "-D_GLIBCXX_DEBUG"]);
    }
    args.push(source.to_str().unwrap());
    args.push("-o");
    args.push(output.to_str().unwrap());
    let mut kid = std::process::Command::new("c++")
        .args(&args)
        .stderr(std::process::Stdio::inherit())
        .spawn().unwrap();
    kid.wait().unwrap();
}

fn run_build(args: &clap::ArgMatches) {
    let source_name = Path::new(args.value_of("input").unwrap());
    assert!(source_name.extension().unwrap() == "cpp");
    let exec_name = source_name.with_extension("e");
    let release = args.is_present("release");
    compile_cpp(source_name, &exec_name, release);
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

fn recursive_find_tests(testdir: &Path) -> Box<Iterator<Item=std::path::PathBuf>> {
	Box::new(walkdir::WalkDir::new(testdir).follow_links(true)
		.into_iter()
		.filter_map(|e| e.ok())
		.map(|entry| entry.path().to_path_buf())
		.filter(|path| path.extension().map(|ext| ext == "in").unwrap_or(false)))
}

fn timefn<T, F: FnOnce() -> T>(f: F) -> (T, std::time::Duration) {
	let inst = std::time::Instant::now();
	let x = f();
	let t = inst.elapsed();
	(x, t)
}

fn timefmt(t: std::time::Duration) -> String {
	format!("{}.{:02}s", t.as_secs(), t.subsec_nanos() / 10000000)
}

fn run_test(args: &clap::ArgMatches, checker: Box<Checker>) {
    let executable = Path::new(args.value_of("exec").unwrap());
    let testdir = Path::new(args.value_of("testdir").unwrap());
    let print_success = !args.is_present("no-print-success");
	let test_count = recursive_find_tests(testdir).count();
	let mut pb = pbr::ProgressBar::new(test_count as u64);
	for ref in_path in recursive_find_tests(testdir) {
        let out_path = in_path.with_extension("out");
		let (kid_out, timing) = timefn(move || {
			exec(executable, StrRes::FilePath(std::path::PathBuf::from(in_path)))
		});
		let timestr = timefmt(timing).blue().bold();
		let correct = checker.check(
			StrRes::FilePath(std::path::PathBuf::from(in_path)),
			StrRes::FilePath(std::path::PathBuf::from(&out_path)),
			kid_out,
		);
		if correct {
			if print_success {
				pb_interwrite!(pb, "{:13} {} {}", "ACCEPT".green().bold(), timestr, in_path.display());
			}
		} else {
			pb_interwrite!(pb, "{:13} {} {}", "WRONG ANSWER".red().bold(), timestr, in_path.display());
		}
		pb.inc();
    }
}

trait Checker {
	fn check(&self, input: StrRes, my_output: StrRes, perfect_output: StrRes) -> bool;
}
struct CheckerDiffOut;
impl Checker for CheckerDiffOut {
	fn check(&self, _input: StrRes, my_output: StrRes, perfect_output: StrRes) -> bool {
		equal_bew(&my_output.get_string(), &perfect_output.get_string())
	}
}
struct CheckerApp {
	app: String,
}
impl Checker for CheckerApp {
	fn check(&self, input: StrRes, my_output: StrRes, perfect_output: StrRes) -> bool {
		input.with_filename(|i_path| {
			my_output.with_filename(|mo_path| {
				perfect_output.with_filename(|po_path| {
					std::process::Command::new(&self.app)
								.args(&[i_path, mo_path, po_path])
								.status().unwrap()
								.success()
				})
			})
		})
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
                .index(1))
            .arg(clap::Arg::with_name("release")
                .short("O")
                .long("release")))
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
		let checker: Box<Checker> = if let Some(checker_app) = subcmd_args.value_of("checker") {
			Box::new(CheckerApp { app: checker_app.to_owned() })
        } else {
			Box::new(CheckerDiffOut {})
        };
		run_test(subcmd_args, checker);
    }
}
