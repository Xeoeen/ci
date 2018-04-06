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
#[structopt(name = "ci", about = "CLI for building and testing programming contest tasks", raw(global_setting = "structopt::clap::AppSettings::VersionlessSubcommands"))]
pub enum Args {
	#[structopt(name = "build", about = "Compile solution with useful flags")]
	Build {
		#[structopt(name = "SOURCE", parse(from_os_str), help = "Path to source file")]
		source: PathBuf,
		#[structopt(short = "O", long = "release", help = "Enable optimizations")]
		release: bool,
		#[structopt(long = "standard", parse(try_from_str = "parse_standard"), default_value = "17", help = "For choosing cpp version")]
		standard: commands::build::CppVer,
	},
	#[structopt(name = "test", about = "Run ans check solution on tests from directory")]
	Test {
		#[structopt(name = "EXECUTABLE", parse(from_os_str), help = "Path to executable")]
		executable: PathBuf,
		#[structopt(name = "TESTDIR", parse(from_os_str), help = "Path to test directory")]
		testdir: PathBuf,
		#[structopt(long = "checker", parse(try_from_str = "parse_checker"), default_value = "\0CheckerDiffOut", help = "Path to checker")]
		checker: Box<checkers::Checker>,
		#[structopt(long = "no-print-success", help = "Do not print successful tests")]
		no_print_success: bool,
	},
	#[structopt(name = "multitest", about = "Run solution on random tests until it fails")]
	Multitest {
		#[structopt(name = "GEN", parse(from_os_str), help = "Path to test generator")]
		gen: PathBuf,
		#[structopt(name = "EXECUTABLES", parse(from_os_str), help = "Paths to executables")]
		executables: Vec<PathBuf>,
		#[structopt(long = "checker", parse(try_from_str = "parse_checker"), default_value = "\0CheckerDiffOut", help = "Path to checker")]
		checker: Box<checkers::Checker>,
	},
	#[structopt(name = "vendor", about = "Merge solution and it's dependencies into single file")]
	Vendor {
		#[structopt(name = "SOURCE", parse(from_os_str), help = "Path to source file")]
		source: PathBuf,
	},
	#[structopt(name = "internal-autocomplete", about = "Generate autocompletion script for appropriate shell")]
	InternalAutocomplete {
		#[structopt(name = "SHELL", parse(try_from_str = "parse_shell"), help = "Shell name")]
		shell: structopt::clap::Shell,
	},
}
