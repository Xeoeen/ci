use error::R;
use std::{fs, path::Path};
use ui::Ui;
use unijudge;
use util::connect;

pub fn run(url: &str, code: &Path, ui: &mut Ui) -> R<()> {
	let tu = unijudge::TaskUrl::deconstruct(url)?;
	let sess = connect(url, ui)?;
	let cont = sess.contest(&tu.contest);
	let langs = cont.languages()?;
	let good_langs = match code.extension().map(|ext| ext.to_str().unwrap()) {
		Some("cpp") => &["C++", "GNU G++17 7.3.0"],
		_ => panic!("unrecognized language"),
	};
	let lang = langs
		.iter()
		.find(|lang| good_langs.contains(&lang.name.as_str()))
		.ok_or_else(|| format_err!("no matching language found for extension {:?}", code.extension()))?;
	let code = fs::read_to_string(code)?;
	cont.submit(&tu.task, lang, &code)?;
	let submissions = cont.submissions_recent()?;
	ui.submit_success(submissions[0].id.clone());
	Ok(())
}
