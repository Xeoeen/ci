use error::R;
use failure::ResultExt;
use ui::Ui;
use unijudge;
use util::{connect, demand_dir, writefile};

pub fn run(url: &str, ui: &Ui) -> R<()> {
	let tu = unijudge::TaskUrl::deconstruct(url);
	let sess = connect(url, ui);
	let cont = sess.contest(&tu.contest);
	demand_dir("./tests/").context("failed to create tests directory")?;
	demand_dir("./tests/example/").context("failed to create tests directory")?;
	let tests = cont.examples(&tu.task);
	for (i, test) in tests.into_iter().enumerate() {
		writefile(&format!("./tests/example/{}.in", i + 1), &test.input);
		writefile(&format!("./tests/example/{}.out", i + 1), &test.output);
	}
	Ok(())
}
