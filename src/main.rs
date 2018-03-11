#[macro_use] extern crate structopt;
extern crate walkdir;
extern crate colored;
extern crate tempfile;
extern crate pbr;
extern crate itertools;

mod checkers;
mod strres;
#[macro_use] mod ui;

use std::path::{Path, PathBuf};
use std::io::{Write};
use colored::*;
use checkers::*;
use strres::{StrRes, exec};
use structopt::StructOpt;
use std::cmp::Ordering;
use itertools::Itertools;

enum CppVer {
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

fn compile_cpp(source: &Path, output: &Path, release: bool, cppver: CppVer) {
    let mut args = vec![];
	args.push(cppver.flag());
	args.extend_from_slice(&["-Wall", "-Wextra", "-Wconversion", "-Wno-sign-conversion"]);
    if release {
        args.push("-O2");
    } else {
        args.extend_from_slice(&["-g", "-D_GLIBCXX_DEBUG", "-fno-sanitize-recover=undefined"]);
    }
    args.push(source.to_str().unwrap());
    args.push("-o");
    args.push(output.to_str().unwrap());
    let mut kid = std::process::Command::new("clang++")
        .args(&args)
        .stderr(std::process::Stdio::inherit())
        .spawn().unwrap();
    assert!(kid.wait().unwrap().success());
}

fn run_build(args: Args) {
	if let Args::Build { source, release, standard } = args {
		assert!(source.extension().unwrap() == "cpp");
		let executable = source.with_extension("e");
		compile_cpp(&source, &executable, release, standard);
	}
}

fn ord_by_test_number(lhs: &std::path::PathBuf, rhs: &std::path::PathBuf) -> Ordering {
	for grp in lhs.to_str().unwrap().chars().group_by(|c| c.is_numeric()).into_iter().zip_longest(rhs.to_str().unwrap().chars().group_by(|c| c.is_numeric()).into_iter()) {
		match grp {
			itertools::EitherOrBoth::Both((isdig, lgrp), (_, rgrp)) => {
				let grp_compr = if isdig {
					let lnum: i64 = lgrp.collect::<String>().parse().unwrap();
					let rnum: i64 = rgrp.collect::<String>().parse().unwrap();
					lnum.cmp(&rnum)
				} else {
					lgrp.cmp(rgrp)
				};
				if grp_compr != Ordering::Equal {
					return grp_compr;
				}
			},
			itertools::EitherOrBoth::Left(_) => return Ordering::Greater,
			itertools::EitherOrBoth::Right(_) => return Ordering::Less,
		}
	}
	Ordering::Equal
}

fn recursive_find_tests(testdir: &Path) -> Box<Iterator<Item=std::path::PathBuf>> {
	let mut tests: Vec<_> = walkdir::WalkDir::new(testdir).follow_links(true)
		.into_iter()
		.filter_map(|e| e.ok())
		.map(|entry| entry.path().to_path_buf())
		.filter(|path| path.extension().map(|ext| ext == "in").unwrap_or(false)).collect();
	tests.sort_by(ord_by_test_number);
	Box::new(tests.into_iter())
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

fn run_test(args: Args) {
	if let Args::Test { executable, testdir, no_print_success, checker } = args {
		diagnose_app(&executable);
		let test_count = recursive_find_tests(&testdir).count();
		let mut pb = pbr::ProgressBar::new(test_count as u64);
		for ref in_path in recursive_find_tests(&testdir) {
			let out_path = in_path.with_extension("out");
			let (outcome, timing) = test_single(&executable, StrRes::from_path(in_path), StrRes::from_path(&out_path), checker.as_ref());
			if outcome != TestResult::Accept || !no_print_success {
				let rstr = outcome.format_long();
				let timestr = timefmt(timing).blue().bold();
				pb_interwrite!(pb, "{} {} {}", rstr, timestr, in_path.display());
			}
			pb.inc();
		}
	}
}

fn run_multitest(args: Args) {
	if let Args::Multitest { gen, executables, checker } = args {
		for ref executable in &executables {
			diagnose_app(executable);
		}
		let mut i = 1;
		loop {
			let test_str = exec(&gen, StrRes::Empty).unwrap();
			print_flush!("(autogenerated {:<6})", i);

			let (out1, t1) = timefn(|| exec(Path::new(&executables[0]), test_str.clone()));
			let out1 = if let Ok(out1) = out1 {
				print_flush!(" {}", timefmt(t1).green().bold());
				out1
			} else {
				print_flush!(" {}", timefmt(t1).red().bold());
				test_str.print_to_stdout();
				break
			};

			let mut all_succeded = true;
			for ref execi in &executables[1..] {
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
}

fn run_genbashauto(args: Args) {
	if let Args::InternalAutocomplete { shell } = args {
		Args::clap().gen_completions_to("ci", shell, &mut std::io::stdout());
	}
}

fn run_vendor(args: Args) {
	if let Args::Vendor { source } = args {
		println!("#include <bits/stdc++.h>");
		std::process::Command::new("g++")
			.args(&["-I", "/usr/share/ci/dummy-includes", "-E", source.as_path().to_str().unwrap()])
			.stdout(std::process::Stdio::inherit())
			.spawn().unwrap()
			.wait().unwrap();
	}
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
enum Args {
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
		Args::Build { .. } => run_build(args),
		Args::Test { .. } => run_test(args),
		Args::Multitest { .. } => run_multitest(args),
		Args::Vendor { .. } => run_vendor(args),
		Args::InternalAutocomplete { .. } => run_genbashauto(args),
	}
}
