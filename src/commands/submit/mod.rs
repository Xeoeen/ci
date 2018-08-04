mod codeforces;
mod sio2;

use error::{E, R};
use failure::Error;
use reqwest::Url;
use std::path::Path;
use ui::Ui;

pub trait Site {
	fn submit_solution(url: &Url, code: &Path, ui: &Ui);
}

type Submitter = fn(&Url, &Path, &Ui);
const MATCHERS: &[(&'static str, Submitter)] = &[
	("sio2.staszic.waw.pl", sio2::Sio2::submit_solution),
	("sio2.mimuw.edu.pl", sio2::Sio2::submit_solution),
	("kiwi.ii.uni.wroc.pl", sio2::Sio2::submit_solution),
	("szkopul.edu.pl", sio2::Sio2::submit_solution),
	("codeforces.com", codeforces::Codeforces::submit_solution),
];

pub fn run(url: &Url, code: &Path, ui: &Ui) -> R<()> {
	let domain = url.domain().unwrap();
	MATCHERS
		.iter()
		.find(|&&(dom, _)| dom == domain)
		.ok_or_else(|| Error::from(E::UnsupportedProblemSite(domain.to_owned())))
		.map(|(_, f)| f(url, code, ui))
}
