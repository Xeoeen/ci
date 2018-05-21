use cli::Args;
use error::*;
use std;
use structopt::{clap::Shell, StructOpt};

pub fn run(shell: Shell) -> R<()> {
	Args::clap().gen_completions_to("ci", shell, &mut std::io::stdout());
	Ok(())
}
