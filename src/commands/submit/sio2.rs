use super::Site;
use reqwest::Url;
use sio2;
use std::{fs::File, path::Path, str::from_utf8};
use strres::endread;
use ui::Ui;

pub struct Sio2;
impl Site for Sio2 {
	fn submit_solution(url: &Url, code: &Path, ui: &Ui) {
		let ps = url.path_segments().unwrap().collect::<Vec<_>>();
		let contest_name = ps[1];
		let problem = ps[3];
		let (user, pass) = ui.read_auth(url.domain().unwrap());
		let site = {
			let mut site = url.to_owned();
			site.path_segments_mut().unwrap().pop().pop().pop().pop().pop().pop();
			site
		};
		let mut sess = sio2::Session::new(site).login(user, pass).spawn();
		let mut contest = sess.contest(contest_name);
		let problem_id = contest
			.submittable_problems()
			.into_iter()
			.find(|prob| prob.symbol == problem)
			.expect("Problem not available for submitting")
			.id;
		let lang = {
			// TODO use file submit api instead
			match code.extension().unwrap().to_str().unwrap() {
				"cpp" | "cc" | "cxx" => "C++",
				"c" => "C",
				"pas" => "Pascal",
				"py" => "Python",
				"rs" => "Rust",
				_ => panic!("Unrecognized language"),
			}
		};
		contest.submit(problem_id, lang, from_utf8(&endread(File::open(code).unwrap()).unwrap()).unwrap());
	}
}
