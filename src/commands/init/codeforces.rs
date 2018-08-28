use super::{Site, Test};
use codeforces as cf;
use commands::init::Description;
use reqwest::Url;
use ui::Ui;

pub struct Codeforces {
	session: cf::Session,
}

pub fn connect(_: &Url, _: &Ui) -> Box<Site> {
	Box::new(Codeforces {
		session: cf::Session::new().unwrap(),
	})
}

impl Site for Codeforces {
	fn download_tests(&mut self, url: &Url, _ui: &Ui) -> Vec<Test> {
		let mut prob = self.session.problem_from_url(url).unwrap();
		prob.example_tests()
			.unwrap()
			.into_iter()
			.map(|test| Test {
				input: test.input,
				output: test.output,
			})
			.collect()
	}

	fn download_description(&mut self, _: &Url, _: &Ui) -> Option<Description> {
		eprintln!("Codeforces init does not support description download yet");
		None
	}
}
