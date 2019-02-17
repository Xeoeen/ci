use error::R;
use failure::ResultExt;
use std::path::Path;
use ui::Ui;
use unijudge;
use util::{connect, demand_dir, writefile};

pub fn run(url: &str, root: &Path, ui: &mut Ui) -> R<()> {
	let tu = unijudge::TaskUrl::deconstruct(url)?;
	let sess = connect(url, ui)?;
	let cont = sess.contest(&tu.contest);
	demand_dir(&root.join("tests")).context("failed to create tests directory")?;
	demand_dir(&root.join("tests/example")).context("failed to create tests directory")?;
	let tests = cont.examples(&tu.task)?;
	for (i, test) in tests.into_iter().enumerate() {
		writefile(&root.join(&format!("tests/example/{}.in", i + 1)), &test.input);
		writefile(&root.join(&format!("tests/example/{}.out", i + 1)), &test.output);
	}
	ui.print_finish_init();
	Ok(())
}
