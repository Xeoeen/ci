use cli::Args;
use std;
use structopt::StructOpt;
use error::*;

pub fn run(args: Args) -> Result<()> {
	if let Args::InternalAutocomplete { shell } = args {
		Args::clap().gen_completions_to("ci", shell, &mut std::io::stdout());
		Ok(())
	}
	else {
		Err(Error::from(CliError::WrongCommand))
	}
}
