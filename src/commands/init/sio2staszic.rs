use super::{Site, Test};
use reqwest::Url;
use sio2;
use std::{collections::HashMap, io::Read};
use tar::Archive;
use ui::read_auth;

pub struct Sio2Staszic;

impl Site for Sio2Staszic {
	fn download_tests(url: &Url) -> Vec<Test> {
		let ps = url.path_segments().unwrap().collect::<Vec<_>>();
		let contest = ps[1];
		let problem = ps[3];

		let (user, pass) = read_auth("sio2.staszic.waw.pl");
		let mut sess = sio2::Session::new("https://sio2.staszic.waw.pl".parse().unwrap()).login(user, pass).spawn();
		let tarfile = sess.get_url(&format!("https://sio2.staszic.waw.pl/c/{}/staszic/example-tests/{}/", contest, problem).parse().unwrap());
		let mut ar = Archive::new(tarfile.as_slice());

		let mut tests: HashMap<String, (Option<String>, Option<String>)> = HashMap::new();
		for file in ar.entries().unwrap() {
			let mut file = file.unwrap();
			let (name, ty) = {
				let path = file.header().path().unwrap();
				let paths = path.to_str().unwrap();
				let name: &str = &paths[..paths.find('.').unwrap()];
				let ty: &str = &paths[paths.find('.').unwrap() + 1..];
				(name.to_owned(), ty.to_owned())
			};
			let mut content = String::new();
			file.read_to_string(&mut content).unwrap();
			let entry = tests.entry(name).or_insert((None, None));
			if ty == "in" {
				entry.0 = Some(content);
			} else if ty == "out" {
				entry.1 = Some(content);
			} else {
				panic!("failed to parse entry (invalid extension)");
			}
		}

		tests
			.into_iter()
			.map(|(_, ss)| Test {
				input: ss.0.unwrap(),
				output: ss.1.unwrap(),
			})
			.collect()
	}
}
