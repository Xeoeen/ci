use auth;
use colored::Colorize;
use keyring::{Keyring, KeyringError};
use reqwest::Url;
use sio2;
use std::{
	self, fs::{create_dir, File}, io::{self, Write}
};
use ui::Ui;

pub fn timefn<T, F: FnOnce() -> T>(f: F) -> (T, std::time::Duration) {
	let inst = std::time::Instant::now();
	let x = f();
	let t = inst.elapsed();
	(x, t)
}
pub fn writefile(path: &str, content: &str) {
	let mut f = File::create(path).unwrap();
	f.write_all(content.as_bytes()).unwrap();
}
pub fn demand_dir(path: &str) -> Result<(), io::Error> {
	match create_dir(path) {
		Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(()),
		r => r,
	}
}

pub fn sio2_get_session(url: &Url, ui: &Ui) -> sio2::Session {
	let sio2::task_url::Deconstructed { site, .. } = sio2::task_url::deconstruct(&url);
	let keyring_name = format!("{} @sessionid", site.domain().unwrap());
	let keyring = Keyring::new("ci", &keyring_name);
	match keyring.get_password() {
		Ok(sessionid) => sio2::Session::new(site.clone()).cached_session(sessionid).spawn(),
		Err(e) => {
			if let KeyringError::NoPasswordFound = e {
			} else {
				eprintln!("{} {}", "keyring error:".red().bold(), e);
			}
			let (user, pass) = auth::get(url.domain().unwrap(), ui);
			let sess = sio2::Session::new(site.clone()).login(user, pass).spawn();
			keyring.set_password(&sess.session_id()).unwrap();
			sess
		},
	}
}
