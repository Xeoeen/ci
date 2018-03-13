#[macro_use] extern crate structopt;
extern crate walkdir;
extern crate colored;
extern crate tempfile;
extern crate pbr;
extern crate itertools;

mod checkers;
mod diagnose;
mod strres;
mod testing;
#[macro_use] mod ui;
mod util;
mod commands;

use std::path::{PathBuf};
use structopt::StructOpt;

fn parse_checker(s: &str) -> Result<Box<checkers::Checker>, i32> {
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
fn parse_standard(s: &str) -> Result<commands::build::CppVer, i32> {
	if s == "17" {
		Ok(commands::build::CppVer::Cpp17)
	} else if s == "11" {
		Ok(commands::build::CppVer::Cpp11)
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
		standard: commands::build::CppVer,
	},
	#[structopt(name = "test")]
	Test {
		#[structopt(name = "EXECUTABLE", parse(from_os_str))]
		executable: PathBuf,
		#[structopt(name = "TESTDIR", parse(from_os_str))]
		testdir: PathBuf,
		#[structopt(long = "checker", parse(try_from_str = "parse_checker"), default_value = "\0CheckerDiffOut")]
		checker: Box<checkers::Checker>,
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
		checker: Box<checkers::Checker>,
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
