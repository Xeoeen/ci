#[macro_use] extern crate structopt;
extern crate walkdir;
extern crate colored;
extern crate tempfile;
extern crate pbr;
extern crate itertools;

mod checkers;
mod strres;
#[macro_use] mod ui;
mod commands;

use std::path::{Path, PathBuf};
use std::io::{Write};
use colored::*;
use checkers::*;
use strres::{StrRes, exec};
use structopt::StructOpt;
use std::cmp::Ordering;

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

fn timefn<T, F: FnOnce() -> T>(f: F) -> (T, std::time::Duration) {
	let inst = std::time::Instant::now();
	let x = f();
	let t = inst.elapsed();
	(x, t)
}

fn timefmt(t: std::time::Duration) -> String {
	format!("{}.{:02}s", t.as_secs(), t.subsec_nanos() / 10000000)
}

fn diagnose_app(app: &Path) {
	if app.extension().map(|ext| ext == "e").unwrap_or(false) && app.with_extension("cpp").exists() {
		let srcfile = app.with_extension("cpp");
		let meta_app = std::fs::metadata(app).unwrap();
		let meta_src = std::fs::metadata(srcfile).unwrap();
		if meta_src.modified().unwrap() > meta_app.modified().unwrap() {
			eprint!("{} .e is older than corresponding .cpp file ({}). Continue?", "WARNING".red().bold(), app.display());
			std::io::stderr().flush().unwrap();
			std::io::stdin().read_line(&mut String::new()).unwrap();
		}
	}
}

#[derive(PartialEq, Eq)]
enum TestResult {
	Accept,
	WrongAnswer,
	RuntimeError,
}
impl TestResult {
	pub fn format_long(&self) -> ColoredString {
		self.apply_color(match self {
			&TestResult::Accept =>       "ACCEPT       ",
			&TestResult::WrongAnswer =>  "WRONG ANSWER ",
			&TestResult::RuntimeError => "RUNTIME ERROR",

		})
	}
	pub fn apply_color(&self, s: &str) -> ColoredString {
		match self {
			&TestResult::Accept => s.green().bold(),
			&TestResult::WrongAnswer => s.red().bold(),
			&TestResult::RuntimeError => s.red().bold(),
		}
	}
}
fn test_single(executable: &Path, input: StrRes, perfect_output: StrRes, checker: &Checker) -> (TestResult, std::time::Duration) {
	let (my_output, timing) = timefn(|| exec(executable, input.clone()));
	(if let Ok(my_output) = my_output {
		if checker.check(input, my_output, perfect_output) {
			TestResult::Accept
		} else {
			TestResult::WrongAnswer
		}
	} else {
		TestResult::RuntimeError
	}, timing)
}

fn parse_checker(s: &str) -> Result<Box<Checker>, i32> {
	if s == "\0CheckerDiffOut" {
		Ok(Box::new(checkers::CheckerDiffOut {}))
	} else {
		Ok(Box::new(checkers::CheckerApp { app: s.to_owned() }))
	}
}
fn parse_shell(s: &str) -> Result<structopt::clap::Shell, i32> {
	if s == "bash" {
		Ok(structopt::clap::Shell::Bash)
	} else {
		Err(0)
	}
}
fn parse_standard(s: &str) -> Result<CppVer, i32> {
	if s == "17" {
		Ok(CppVer::Cpp17)
	} else if s == "11" {
		Ok(CppVer::Cpp11)
	} else {
		Err(0)
	}
}

#[derive(StructOpt)]
#[structopt(name = "ci", about = "CLI for building and testing programming contest tasks")]
pub enum Args {
	#[structopt(name = "build")]
	Build {
		#[structopt(name = "SOURCE", parse(from_os_str))]
		source: PathBuf,
		#[structopt(short = "O", long = "release")]
		release: bool,
		#[structopt(long = "standard", parse(try_from_str = "parse_standard"), default_value = "17")]
		standard: CppVer,
	},
	#[structopt(name = "test")]
	Test {
		#[structopt(name = "EXECUTABLE", parse(from_os_str))]
		executable: PathBuf,
		#[structopt(name = "TESTDIR", parse(from_os_str))]
		testdir: PathBuf,
		#[structopt(long = "checker", parse(try_from_str = "parse_checker"), default_value = "\0CheckerDiffOut")]
		checker: Box<Checker>,
		#[structopt(long = "no-print-success")]
		no_print_success: bool,
	},
	#[structopt(name = "multitest")]
	Multitest {
		#[structopt(name = "GEN", parse(from_os_str))]
		gen: PathBuf,
		#[structopt(name = "EXECUTABLES", parse(from_os_str))]
		executables: Vec<PathBuf>,
		#[structopt(long = "checker", parse(try_from_str = "parse_checker"), default_value = "\0CheckerDiffOut")]
		checker: Box<Checker>,
	},
	#[structopt(name = "vendor")]
	Vendor {
		#[structopt(name = "SOURCE", parse(from_os_str))]
		source: PathBuf,
	},
	#[structopt(name = "internal-autocomplete")]
	InternalAutocomplete {
		#[structopt(name = "SHELL", parse(try_from_str = "parse_shell"))]
		shell: structopt::clap::Shell,
	},
}

fn main() {
	let args = Args::from_args();
	match args {
		Args::Build { .. } => commands::build::run(args),
		Args::Test { .. } => commands::test::run(args),
		Args::Multitest { .. } => commands::multitest::run(args),
		Args::Vendor { .. } => commands::vendor::run(args),
		Args::InternalAutocomplete { .. } => commands::genbashauto::run(args),
	}
}
