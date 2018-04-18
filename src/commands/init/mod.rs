mod codeforces;
mod oioioi;

use super::super::cli::Args;
use sio2::Url;
use std::fs::create_dir;
use util::writefile;

pub struct Test {
	input: String,
	output: String,
}

pub trait Site {
	fn download_tests(url: &str) -> Vec<Test>;
}

pub fn run(args: Args) {
	if let Args::Init { url } = args {
		create_dir("./tests/").ok();
		create_dir("./tests/example/").ok();
		let tests = acquire_tests(&url);
		for (i, test) in tests.into_iter().enumerate() {
			writefile(&format!("./tests/example/{}.in", i+1), &test.input);
			writefile(&format!("./tests/example/{}.out", i+1), &test.output);
		}
	}
}

const MATCHERS: &[(&'static str, fn(&str) -> Vec<Test>)] = &[
	("codeforces.com", codeforces::Codeforces::download_tests),
	("sio2.staszic.waw.pl", oioioi::Oioioi::download_tests),
];

fn acquire_tests(url: &str) -> Vec<Test> {
	let parsed = Url::parse(url).unwrap();
	let domain = parsed.domain().unwrap();
	MATCHERS.iter().find(|&&(dom, _)| dom == domain).unwrap().1(url)
}