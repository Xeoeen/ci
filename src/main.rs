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
mod cli;

use structopt::StructOpt;
use cli::Args;

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
