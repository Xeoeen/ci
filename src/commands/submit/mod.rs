use error::R;
use std::path::Path;
use ui::Ui;
use unijudge;
use util::connect;

pub fn run(url: &str, code: &Path, ui: &Ui) -> R<()> {
	let tu = unijudge::TaskUrl::deconstruct(url);
	let sess = connect(url, ui);
	let cont = sess.contest(&tu.contest);
	unimplemented!()
}
