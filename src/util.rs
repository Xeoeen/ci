use auth;
use keyring::{Keyring, KeyringError};
use std::{
	self, fs::{create_dir, File}, io::{self, Write}
};
use ui::Ui;
use unijudge;

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

pub fn connect(url: &str, ui: &mut Ui) -> Box<unijudge::Session> {
	let tu = unijudge::TaskUrl::deconstruct(url);
	let keyring_name = format!("{} @sessionid", tu.site);
	let keyring = Keyring::new("ci", &keyring_name);
	match keyring.get_password() {
		Ok(session_id) => unijudge::connect_cached(&tu.site, &session_id),
		Err(KeyringError::NoPasswordFound) => {
			let (user, pass) = auth::get(&tu.site, ui);
			let sess = unijudge::connect_login(&tu.site, &user, &pass);
			if let Some(session_id) = sess.cache_sessionid() {
				keyring.set_password(&session_id).unwrap();
			} else {
				ui.notice("could not cache session, expect slow connecting");
			}
			sess
		},
		Err(e) => Err(e).unwrap(),
	}
}
