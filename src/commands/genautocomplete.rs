use cli::Args;
use std;
use structopt::{
	StructOpt,
	clap::Shell,
};
use error::*;

pub fn run(shell: Shell) -> R<()> {
	Args::clap().gen_completions_to("ci", shell, &mut std::io::stdout());
	Ok(())
}
