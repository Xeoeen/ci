mod sio2;

use error::{E, R};
use reqwest::Url;
use ui::Ui;

#[derive(Serialize)]
pub struct Resource {
	pub name: String,
	pub description: String,
	pub filename: String,
	pub id: String,
}

pub trait Site {
	fn fetch_resource_list(&mut self) -> Vec<Resource>;
}

type Connector = fn(&Url, &Ui) -> Box<Site>;
const MATCHERS: &[(&str, Connector)] = &[
	("sio2.staszic.waw.pl", sio2::connect),
	("sio2.mimuw.edu.pl", sio2::connect),
	("kiwi.ii.uni.wroc.pl", sio2::connect),
	("szkopul.edu.pl", sio2::connect),
	("codeforces.com", no_resources),
];

pub fn run(url: &Url, ui: &Ui) -> R<()> {
	let domain = url.domain().unwrap();
	let (_, connector) = MATCHERS
		.iter()
		.find(|&&(dom, _)| dom == domain)
		.ok_or_else(|| E::UnsupportedProblemSite(domain.to_owned()))?;
	let mut site = connector(url, ui);
	let resources = site.fetch_resource_list();
	ui.print_resource_list(&resources);
	Ok(())
}

struct NoResources;
impl Site for NoResources {
	fn fetch_resource_list(&mut self) -> Vec<Resource> {
		Vec::new()
	}
}
fn no_resources(_: &Url, _: &Ui) -> Box<Site> {
	Box::new(NoResources)
}
