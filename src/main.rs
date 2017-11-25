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

fn run_build(source: &Path, release: bool) {
    assert!(source.extension().unwrap() == "cpp");
    let executable = source.with_extension("e");
    compile_cpp(source, &executable, release);
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

#[derive(PartialEq, Eq)]
enum TestResult {
	Accept,
	WrongAnswer,
}
impl TestResult {
	pub fn format_long(&self) -> ColoredString {
		self.apply_color(match self {
			&TestResult::Accept =>      "ACCEPT      ",
			&TestResult::WrongAnswer => "WRONG ANSWER",
		})
	}
	pub fn apply_color(&self, s: &str) -> ColoredString {
		match self {
			&TestResult::Accept => s.green().bold(),
			&TestResult::WrongAnswer => s.red().bold(),
		}
	}
}
fn test_single(executable: &Path, input: StrRes, perfect_output: StrRes, checker: &Checker) -> (TestResult, std::time::Duration) {
	let (my_output, timing) = timefn(|| exec(executable, input.clone()));
	(if checker.check(input, my_output, perfect_output) {
		TestResult::Accept
	} else {
		TestResult::WrongAnswer
	}, timing)
}

fn run_test(executable: &Path, testdir: &Path, print_success: bool, checker: &Checker) {
	let test_count = recursive_find_tests(testdir).count();
	let mut pb = pbr::ProgressBar::new(test_count as u64);
	for ref in_path in recursive_find_tests(testdir) {
        let out_path = in_path.with_extension("out");
		let (outcome, timing) = test_single(executable, StrRes::from_path(in_path), StrRes::from_path(&out_path), checker);
		if outcome != TestResult::Accept || print_success {
			let rstr = outcome.format_long();
			let timestr = timefmt(timing).blue().bold();
			pb_interwrite!(pb, "{} {} {}", rstr, timestr, in_path.display());
		}
		pb.inc();
    }
}

macro_rules! print_flush {
	($fmt:expr) => {
		print!($fmt);
		std::io::stdout().flush().unwrap();
	};
	($fmt:expr $(,$arg:expr)*) => {
		print!($fmt $(,$arg)*);
		std::io::stdout().flush().unwrap();
	};
}

fn run_multitest(gen: &Path, execs: &[&Path], checker: Box<Checker>) {
	let mut i = 1;
	loop {

		let test_str = exec(gen, StrRes::Empty);
		print_flush!("(autogenerated {:<6})", i);

		let (out1, t1) = timefn(|| exec(Path::new(&execs[0]), test_str.clone()));
		print_flush!(" {}", timefmt(t1).green().bold());

		let mut all_succeded = true;
		for ref execi in &execs[1..] {

			let (outi, ti) = test_single(execi, test_str.clone(), out1.clone(), checker.as_ref());
			let msg = outi.apply_color(&timefmt(ti));
			print_flush!(" {}", msg);

			if outi != TestResult::Accept {
				all_succeded = false;
			}

		}
		println!("");
		if !all_succeded {
			test_str.print_to_stdout();
			break
		}
		i += 1;
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
        .subcommand(clap::SubCommand::with_name("multitest")
			.arg(clap::Arg::with_name("gen")
				.value_name("GEN")
				.required(true)
				.index(1))
            .arg(clap::Arg::with_name("execs")
                .value_name("EXECS")
                .required(true)
				.multiple(true)
                .index(2))
            .arg(clap::Arg::with_name("checker")
                .long("checker")
                .takes_value(true)))
        .get_matches();
    if let Some(subcmd_args) = args.subcommand_matches("build") {
        let source = Path::new(subcmd_args.value_of("input").unwrap());
		let release = subcmd_args.is_present("release");
		run_build(source, release)
    } else if let Some(subcmd_args) = args.subcommand_matches("test") {
		let executable = Path::new(subcmd_args.value_of("exec").unwrap());
		let testdir = Path::new(subcmd_args.value_of("testdir").unwrap());
		let checker: Box<Checker> = if let Some(checker_app) = subcmd_args.value_of("checker") {
			Box::new(CheckerApp { app: checker_app.to_owned() })
        } else {
			Box::new(CheckerDiffOut {})
        };
		let print_success = !subcmd_args.is_present("no-print-success");
		run_test(executable, testdir, print_success, checker.as_ref())
    } else if let Some(subcmd_args) = args.subcommand_matches("multitest") {
		let gen = Path::new(subcmd_args.value_of("gen").unwrap());
		let execs = subcmd_args.values_of("execs").unwrap().map(|executable| Path::new(executable)).collect::<Vec<_>>();
		let checker: Box<Checker> = if let Some(checker_app) = subcmd_args.value_of("checker") {
			Box::new(CheckerApp { app: checker_app.to_owned() })
        } else {
			Box::new(CheckerDiffOut {})
        };
		run_multitest(gen, &execs, checker)
    }
}
