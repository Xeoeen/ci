#[macro_use]
extern crate structopt;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate itertools;
extern crate keyring;
extern crate pbr;
extern crate reqwest;
extern crate rpassword;
extern crate select;
extern crate sio2;
extern crate tar;
extern crate tempfile;
extern crate term_painter;
extern crate wait_timeout;
extern crate walkdir;

mod checkers;
mod diagnose;
mod error;
mod strres;
mod testing;
#[macro_use]
mod ui;
mod cli;
mod commands;
mod fitness;
mod util;

use cli::Args;
use commands::build::Codegen;
use error::*;
use std::borrow::Borrow;
use structopt::StructOpt;
use term_painter::{
	Color::{Red, Yellow}, ToStyle
};

fn run() -> R<()> {
	let args = Args::from_args();
	match args {
		Args::Build {
			source,
			release,
			profile,
			standard,
		} => {
			let codegen = match (release, profile) {
				(false, false) => Codegen::Debug,
				(true, false) => Codegen::Release,
				(false, true) => Codegen::Profile,
				(true, true) => return Err(format_err!("both --release and --profile specified")),
			};
			commands::build::run(source.as_path(), &codegen, &standard)
		},
		Args::Test {
			executable,
			testdir,
			checker,
			no_print_success,
		} => commands::test::run(executable.as_path(), testdir.as_path(), checker.borrow(), no_print_success),
		Args::Multitest {
			gen,
			executables,
			checker,
			count,
			fitness,
			time_limit,
		} => commands::multitest::run(gen.as_path(), &executables, checker.as_ref(), count, fitness.borrow(), time_limit),
		Args::Vendor { source } => commands::vendor::run(source.as_path()),
		Args::InternalAutocomplete { shell } => commands::genautocomplete::run(shell),
		Args::Init { url } => commands::init::run(&url),
	}
}

fn main() {
	if let Err(e) = run() {
		let error_prefix = Red.bold().paint("error");
		let cause_prefix = Yellow.bold().paint("caused by");
		println!("{}: {}", error_prefix, e);
		for cause in e.causes().skip(1) {
			println!("{}: {}", cause_prefix, cause);
		}
		::std::process::exit(1);
	}
}
