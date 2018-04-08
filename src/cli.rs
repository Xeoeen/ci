use structopt;
use checkers;
use commands;
use std::path::{PathBuf};
use error::*;


fn parse_checker(s: &str) -> Result<Box<checkers::Checker>> {
	if s == "\0CheckerDiffOut" {
		Ok(Box::new(checkers::CheckerDiffOut {}))
	} else {
		Ok(Box::new(checkers::CheckerApp { app: s.to_owned() }))
	}
}

fn parse_standard(s: &str) -> Result<commands::build::CppVer> {
	if s == "17" {
		Ok(commands::build::CppVer::Cpp17)
	} else if s == "11" {
		Ok(commands::build::CppVer::Cpp11)
	} else {
		Err(Error::from(ParseError{}))
	}
}

#[derive(StructOpt)]
#[structopt(name = "ci", about = "CLI for building and testing programming contest tasks", raw(global_setting = "structopt::clap::AppSettings::VersionlessSubcommands"))]
pub enum Args {
	#[structopt(name = "build", about = "Compile source with useful flags")]
	Build {
		#[structopt(name = "SOURCE", parse(from_os_str), help = "Source file path")]
		source: PathBuf,
		#[structopt(short = "O", long = "release", help = "Enable optimizations")]
		release: bool,
		#[structopt(long = "standard", parse(try_from_str = "parse_standard"), default_value = "17", help = "C++ standard")]
		standard: commands::build::CppVer,
	},
	#[structopt(name = "test", about = "Run solution on tests from directory")]
	Test {
		#[structopt(name = "EXECUTABLE", parse(from_os_str), help = "Solution executable path")]
		executable: PathBuf,
		#[structopt(name = "TESTDIR", parse(from_os_str), help = "Tests directory path")]
		testdir: PathBuf,
		#[structopt(long = "checker", parse(try_from_str = "parse_checker"), default_value = "\0CheckerDiffOut", help = "Checker path")]
		checker: Box<checkers::Checker>,
		#[structopt(long = "no-print-success", help = "Do not print successful tests")]
		no_print_success: bool,
	},
	#[structopt(name = "multitest", about = "Run solutions on random tests until they fail")]
	Multitest {
		#[structopt(name = "GEN", parse(from_os_str), help = "Test generator path")]
		gen: PathBuf,
		#[structopt(name = "EXECUTABLES", parse(from_os_str), help = "Solution executables' paths")]
		executables: Vec<PathBuf>,
		#[structopt(long = "checker", parse(try_from_str = "parse_checker"), default_value = "\0CheckerDiffOut", help = "Checker path")]
		checker: Box<checkers::Checker>,
		#[structopt(short = "n", long = "count", help = "Test case count")]
		count: Option<i64>,
	},
	#[structopt(name = "vendor", about = "Merge solution and its dependencies into single source file")]
	Vendor {
		#[structopt(name = "SOURCE", parse(from_os_str), help = "Solution source path")]
		source: PathBuf,
	},
	#[structopt(name = "internal-autocomplete", about = "Generate autocompletion script for appropriate shell")]
	InternalAutocomplete {
		#[structopt(name = "SHELL", help = "Shell name")]
		shell: structopt::clap::Shell,
	},
}
