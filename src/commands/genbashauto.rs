use super::super::*;

pub fn run(args: Args) {
	if let Args::InternalAutocomplete { shell } = args {
		Args::clap().gen_completions_to("ci", shell, &mut std::io::stdout());
	}
}