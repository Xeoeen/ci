use super::{Site, Test};
use codeforces as cf;
use reqwest::Url;
use ui::Ui;

pub struct Codeforces;

impl Site for Codeforces {
	fn download_tests(url: &Url, ui: &Ui) -> Vec<Test> {
		let mut sess = cf::Session::new().unwrap();
		let mut prob = sess.problem_from_url(url).unwrap();
		prob.example_tests()
			.unwrap()
			.into_iter()
			.map(|test| Test {
				input: test.input,
				output: test.output,
			})
			.collect()
	}
}
