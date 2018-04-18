mod codeforces;
mod oioioi;

use sio2::Url;
use std::fs::create_dir;
use util::writefile;
use error::*;

pub struct Test {
	input: String,
	output: String,
}

pub trait Site {
	fn download_tests(url: &Url) -> Vec<Test>;
}

pub fn run(url: Url) -> Result<()> {
	create_dir("./tests/").ok();
	create_dir("./tests/example/").ok();
	let tests = acquire_tests(&url);
	for (i, test) in tests.into_iter().enumerate() {
		writefile(&format!("./tests/example/{}.in", i+1), &test.input);
		writefile(&format!("./tests/example/{}.out", i+1), &test.output);
	}
	Ok(())
}

const MATCHERS: &[(&'static str, fn(&Url) -> Vec<Test>)] = &[
	("codeforces.com", codeforces::Codeforces::download_tests),
	("sio2.staszic.waw.pl", oioioi::Oioioi::download_tests),
];

fn acquire_tests(url: &Url) -> Vec<Test> {
	let domain = url.domain().unwrap();
	MATCHERS.iter().find(|&&(dom, _)| dom == domain).unwrap().1(url)
}