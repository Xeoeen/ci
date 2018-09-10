mod sio2;

use error::{E, R};
use reqwest::Url;
use std::{fs, path::Path};
use ui::Ui;

pub trait Site {
	fn fetch_resource(&mut self, id: &str) -> Vec<u8>;
}

type Connector = fn(&Url, &Ui) -> Box<Site>;
const MATCHERS: &[(&str, Connector)] = &[
	("sio2.staszic.waw.pl", sio2::connect),
	("sio2.mimuw.edu.pl", sio2::connect),
	("kiwi.ii.uni.wroc.pl", sio2::connect),
	("szkopul.edu.pl", sio2::connect),
];

pub fn run(url: &Url, id: &str, file: &Path, ui: &Ui) -> R<()> {
	let domain = url.domain().unwrap();
	let (_, connector) = MATCHERS
		.iter()
		.find(|&&(dom, _)| dom == domain)
		.ok_or_else(|| E::UnsupportedProblemSite(domain.to_owned()))?;
	let mut site = connector(url, ui);
	let resource = site.fetch_resource(id);
	fs::write(file, resource)?;
	Ok(())
}
