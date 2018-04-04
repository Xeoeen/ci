use structopt;
use checkers;
use commands;
use std::path::{PathBuf};


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
		#[structopt(short = "n", long = "count")]
		count: Option<i64>,
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
