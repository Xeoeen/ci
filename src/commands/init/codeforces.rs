use super::{Site, Test};
use reqwest;
use select::{document::Document, predicate::Class};

pub struct Codeforces;

impl Site for Codeforces {
	fn download_tests(url: &str) -> Vec<Test> {
		let text = reqwest::get(url).unwrap().text().unwrap();
		let doc = Document::from(text.as_str());
		let samples = doc.find(Class("sample-test")).next().unwrap();
		let set_count = samples.children().count() / 2;
		let mut tests: Vec<_> = (0..set_count).map(|_| Test { input: String::new(), output: String::new() }).collect();
		for (kid, i) in samples.children().zip(0..) {
			let mut parsed = String::new();
			let testdiv = kid.children().skip(1).next().unwrap();
			for (line, j) in testdiv.children().zip(0..) {
				if j % 2 == 0 {
					parsed += &(line.text() + "\n");
				}
			}
			if i % 2 == 0 {
				tests[i/2].input = parsed;
			} else {
				tests[i/2].output = parsed;
			}
		}
		for test in &tests {
			assert_ne!(test.input, "");
			assert_ne!(test.output, "");
		}
		tests
	}
}