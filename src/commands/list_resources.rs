use error::R;
use ui::Ui;
use unijudge;
use util::connect;

#[derive(Serialize)]
pub struct Resource {
	pub name: String,
	pub description: String,
	pub filename: String,
	pub id: String,
}

pub fn run(url: &str, ui: &mut Ui) -> R<()> {
	let tu = unijudge::TaskUrl::deconstruct(url)?;
	let sess = connect(url, ui)?;
	let cont = sess.contest(&tu.contest);
	let resources = cont
		.resources()?
		.into_iter()
		.map(|rsrc| Resource {
			name: rsrc.name,
			description: rsrc.description,
			filename: rsrc.filename,
			id: rsrc.id,
		})
		.collect::<Vec<_>>();
	ui.print_resource_list(&resources);
	Ok(())
}
