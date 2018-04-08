#[macro_use] extern crate structopt;
#[macro_use] extern crate failure;
#[macro_use] extern crate failure_derive;
extern crate walkdir;
extern crate colored;
extern crate tempfile;
extern crate pbr;
extern crate itertools;

mod error;
mod checkers;
mod diagnose;
mod strres;
mod testing;
#[macro_use] mod ui;
mod util;
mod commands;
mod cli;

use structopt::StructOpt;
use cli::Args;
use colored::Colorize;
use error::*;

fn run() -> Result<()> {
	let args = Args::from_args();
	match args {
		Args::Build { .. } => commands::build::run(args),
		Args::Test { .. } => commands::test::run(args),
		Args::Multitest { .. } => commands::multitest::run(args),
		Args::Vendor { .. } => commands::vendor::run(args),
		Args::InternalAutocomplete { .. } => commands::genautocomplete::run(args),
	}
}

fn main() {
	if let Err(e) = run() {
		let error_prefix = "error".red().bold();
		let cause_prefix = "caused by".yellow().bold();
		println!("{}: {}",error_prefix, e);
		for cause in e.causes().skip(1) {
			println!("{}: {}", cause_prefix, cause);
		}
		::std::process::exit(1);
	}
}
