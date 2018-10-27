#[macro_use]
extern crate structopt;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate itertools;
extern crate keyring;
extern crate rpassword;
extern crate serde;
extern crate serde_json;
extern crate tempfile;
extern crate term;
extern crate unijudge;
extern crate wait_timeout;
extern crate walkdir;

mod auth;
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

use cli::{Args, Command};
use commands::build::Codegen;
use error::*;
use std::{borrow::Borrow, ops::DerefMut};
use structopt::StructOpt;
use ui::Ui;

fn run(command: Command, ui: &mut Ui) -> R<()> {
	match command {
		Command::Build {
			source,
			release,
			profile,
			standard,
			library,
		} => {
			let codegen = match (release, profile) {
				(false, false) => Codegen::Debug,
				(true, false) => Codegen::Release,
				(false, true) => Codegen::Profile,
				(true, true) => return Err(format_err!("both --release and --profile specified")),
			};
			commands::build::run(source.as_path(), &codegen, &standard, library.as_ref().map(|p| p.as_path()))
		},
		Command::Test {
			executable,
			testdir,
			checker,
			no_print_success,
			print_output,
		} => commands::test::run(executable.as_path(), testdir.as_path(), checker.borrow(), no_print_success, print_output, ui),
		Command::Multitest {
			gen,
			executables,
			checker,
			count,
			fitness,
			time_limit,
			ignore_generator_fail,
		} => commands::multitest::run(
			gen.as_path(),
			&executables,
			checker.as_ref(),
			count,
			fitness.borrow(),
			time_limit,
			ignore_generator_fail,
			ui,
		),
		Command::Vendor { source } => commands::vendor::run(source.as_path(), ui),
		Command::GenerateAutocomplete { shell } => commands::genautocomplete::run(shell),
		Command::Init { url } => commands::init::run(&url, ui),
		Command::Submit { source, url } => commands::submit::run(&url, &source, ui),
		Command::ListResources { url } => commands::list_resources::run(&url, ui),
		Command::Download { url, id, file } => commands::download::run(&url, &id, &file, ui),
		Command::TrackSubmit { url, id, sleep_duration } => commands::tracksubmit::run(&url, id, sleep_duration, ui),
	}
}

fn main() {
	let args = Args::from_args();
	let Args { mut ui, command } = args;
	if let Err(e) = run(command, ui.deref_mut()) {
		ui.print_error(e);
		::std::process::exit(1);
	}
}
