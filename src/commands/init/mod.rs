mod codeforces;
mod sio2staszic;

use error::*;
use reqwest::Url;
use ui::Ui;
use util::{demand_dir, writefile};

pub struct Test {
	input: String,
	output: String,
}

pub trait Site {
	fn download_tests(&mut self, url: &Url, ui: &Ui) -> Vec<Test>;
}

pub fn run(url: &Url, ui: &Ui) -> R<()> {
	let domain = url.domain().unwrap();
	let mut site: Box<Site> = MATCHERS
		.iter()
		.find(|&&(dom, _)| dom == domain)
		.ok_or_else(|| Error::from(E::UnsupportedProblemSite(domain.to_owned())))
		.map(move |(_, f)| f(url, ui))?;
	demand_dir("./tests/").context("failed to create tests directory")?;
	demand_dir("./tests/example/").context("failed to create tests directory")?;
	let tests = site.download_tests(url, ui);
	for (i, test) in tests.into_iter().enumerate() {
		writefile(&format!("./tests/example/{}.in", i + 1), &test.input);
		writefile(&format!("./tests/example/{}.out", i + 1), &test.output);
	}
	Ok(())
}

type Connector = fn(&Url, &Ui) -> Box<Site>;
const MATCHERS: &[(&str, Connector)] = &[("codeforces.com", codeforces::connect), ("sio2.staszic.waw.pl", sio2staszic::connect)];
