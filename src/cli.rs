use checkers;
use commands;
use error::*;
use fitness;
use std::{path::PathBuf, time::Duration};
use structopt;
use ui;

fn parse_checker(s: &str) -> R<Box<checkers::Checker>> {
	if s == "\0CheckerDiffOut" {
		Ok(Box::new(checkers::CheckerDiffOut {}))
	} else {
		Ok(Box::new(checkers::CheckerApp::new(s.to_owned())?))
	}
}

fn parse_standard(s: &str) -> R<commands::build::CppVer> {
	if s == "17" {
		Ok(commands::build::CppVer::Cpp17)
	} else if s == "11" {
		Ok(commands::build::CppVer::Cpp11)
	} else {
		Err(Error::from(ParseError {
			expected: "{17, 11}",
			found: s.to_owned(),
		}))
	}
}

fn parse_duration(s: &str) -> R<Duration> {
	let sufstart = s.find(|c: char| !c.is_digit(10)).ok_or_else(|| format_err!("time unit not found(for example, 10s)"))?;
	let n = s[..sufstart].parse()?;
	let suf = &s[sufstart..];
	Ok(match suf {
		"h" => Duration::from_secs(n * 60 * 60),
		"min" => Duration::from_secs(n * 60),
		"s" => Duration::from_secs(n),
		"ms" => Duration::from_millis(n),
		_ => return Err(format_err!("unsupported time unit {} (supported: h, min, s, ms)", suf)),
	})
}

fn parse_ui(s: &str) -> R<Box<ui::Ui>> {
	Ok(match s {
		"human" => Box::new(ui::Human::new()),
		"json" => Box::new(ui::Json::new()),
		_ => return Err(format_err!("unknown format {} (known: human, json)", s)),
	})
}

#[derive(StructOpt)]
#[structopt(
	name = "ci",
	about = "CLI for building and testing programming contest tasks",
	raw(global_setting = "structopt::clap::AppSettings::VersionlessSubcommands")
)]
pub struct Args {
	#[structopt(long = "format", parse(try_from_str = "parse_ui"), default_value = "human", help = "User interface format")]
	pub ui: Box<ui::Ui>,
	#[structopt(subcommand)]
	pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
	#[structopt(name = "build", about = "Compile source with useful flags")]
	Build {
		#[structopt(name = "SOURCE", parse(from_os_str), help = "Source file path")]
		source: PathBuf,
		// TODO force structopt so that these are mutually exclusive
		#[structopt(short = "O", long = "release", help = "Enable optimizations")]
		release: bool,
		#[structopt(long = "profile", help = "Enable profiling")]
		profile: bool,
		#[structopt(long = "standard", parse(try_from_str = "parse_standard"), default_value = "17", help = "C++ standard")]
		standard: commands::build::CppVer,
		#[structopt(long = "lib", parse(from_os_str), help = "Additional library code")]
		library: Option<PathBuf>,
	},
	#[structopt(name = "test", about = "Run solution on tests from directory")]
	Test {
		#[structopt(name = "EXECUTABLE", parse(from_os_str), help = "Solution executable path")]
		executable: PathBuf,
		#[structopt(name = "TESTDIR", parse(from_os_str), help = "Tests directory path")]
		testdir: PathBuf,
		#[structopt(
			long = "checker",
			parse(try_from_str = "parse_checker"),
			default_value = "\0CheckerDiffOut",
			help = "Checker path"
		)]
		checker: Box<checkers::Checker>,
		#[structopt(long = "no-print-success", help = "Do not print successful tests")]
		no_print_success: bool,
		#[structopt(long = "print-output", help = "Print output")]
		print_output: bool,
	},
	#[structopt(name = "multitest", about = "Run solutions on random tests until they fail")]
	Multitest {
		#[structopt(name = "GEN", parse(from_os_str), help = "Test generator path")]
		gen: PathBuf,
		#[structopt(name = "EXECUTABLES", parse(from_os_str), help = "Solution executables' paths")]
		executables: Vec<PathBuf>,
		#[structopt(
			long = "checker",
			parse(try_from_str = "parse_checker"),
			default_value = "\0CheckerDiffOut",
			help = "Checker path"
		)]
		checker: Box<checkers::Checker>,
		#[structopt(short = "n", long = "count", help = "Test case count")]
		count: Option<i64>,
		// TODO force structopt to require count
		#[structopt(
			long = "fitness",
			parse(try_from_str = "fitness::parse_fitness"),
			default_value = "@bytelen",
			help = "Test fitness function"
		)]
		fitness: Box<fitness::Fitness>,
		#[structopt(long = "time-limit", parse(try_from_str = "parse_duration"), help = "Program execution time limit")]
		time_limit: Option<Duration>,
		#[structopt(long = "ignore-gen-fail", help = "Ignore non-zero test generator exit code")]
		ignore_generator_fail: bool,
	},
	#[structopt(name = "vendor", about = "Merge solution and its dependencies into single source file")]
	Vendor {
		#[structopt(name = "SOURCE", parse(from_os_str), help = "Solution source path")]
		source: PathBuf,
	},
	#[structopt(name = "init", about = "Set up working environment and download tests")]
	Init {
		#[structopt(name = "URL", help = "Task description URL")]
		url: String,
	},
	#[structopt(name = "submit", about = "Submit solution to programming contest")]
	Submit {
		#[structopt(name = "SOURCE", parse(from_os_str), help = "Solution source path")]
		source: PathBuf,
		#[structopt(name = "URL", help = "Task description URL")]
		url: String,
	},
	#[structopt(name = "track-submit", about = "Check submission status automatically")]
	TrackSubmit {
		#[structopt(name = "URL", help = "Task description URL")]
		url: String,
		#[structopt(name = "SUBMID", help = "Submission ID")]
		id: String,
		#[structopt(name = "SLEEPTIME", parse(try_from_str = "parse_duration"), help = "Time to sleep for between checks")]
		sleep_duration: Duration,
	},
	#[structopt(name = "list-resources", about = "List provided resources")]
	ListResources {
		#[structopt(name = "URL", help = "Task description URL")]
		url: String,
	},
	#[structopt(name = "download", about = "Download a resource")]
	Download {
		#[structopt(name = "URL", help = "Task description URL")]
		url: String,
		#[structopt(name = "ID", help = "Resource id")]
		id: String,
		#[structopt(name = "FILE", parse(from_os_str), help = "Target filename")]
		file: PathBuf,
	},
	#[structopt(name = "generate-autocomplete", about = "Generate autocompletion script for appropriate shell")]
	GenerateAutocomplete {
		#[structopt(name = "SHELL", help = "Shell name")]
		shell: structopt::clap::Shell,
	},
}
