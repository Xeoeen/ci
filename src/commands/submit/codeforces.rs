use super::Site;
use codeforces as cf;
use reqwest::Url;
use std::{fs::File, io::Read, path::Path};
use ui::Ui;

pub struct Codeforces;
impl Site for Codeforces {
	fn submit_solution(url: &Url, path: &Path, ui: &Ui) {
		let (user, pass) = ui.read_auth("codeforces.com");
		let code = {
			let mut code = String::new();
			let mut f = File::open(path).unwrap();
			f.read_to_string(&mut code).unwrap();
			code
		};
		let lang_name = match path.extension().unwrap().to_str().unwrap() {
			"cpp" | "cxx" | "cc" => "GNU G++17 7.3.0",
			"c" => "GNU GCC C11 5.1.0",
			"hs" => "Haskell GHC 7.8.3",
			"rs" => "Rust 1.26",
			_ => panic!("unrecognized file extension"),
		};
		let mut sess = cf::Session::new().unwrap();
		sess.login(&user, &pass).unwrap();
		let mut prob = sess.problem_from_url(url).unwrap();
		let language_id = prob
			.allowed_languages()
			.unwrap()
			.into_iter()
			.find(|lang| lang.name == lang_name)
			.expect("language identifiers became outdated - program update required")
			.id;
		prob.submit(&code, language_id).unwrap();
	}
}
