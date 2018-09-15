use error::R;
use std::{fs, path::Path};
use ui::Ui;
use unijudge;
use util::connect;

pub fn run(url: &str, id: &str, file: &Path, ui: &Ui) -> R<()> {
	let tu = unijudge::TaskUrl::deconstruct(url);
	let sess = connect(url, ui);
	let cont = sess.contest(&tu.contest);
	let resources = cont.resources();
	let resource = resources.iter().find(|rsrc| rsrc.id == id).unwrap();
	let contents = cont.resource_fetch(resource);
	fs::write(file, contents)?;
	Ok(())
}
