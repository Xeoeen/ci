use keyring::{Keyring, KeyringError};
use ui::Ui;

pub fn get(domain: &str, ui: &mut Ui) -> (String, String) {
	let key = Keyring::new("ci", domain);
	match key.get_password() {
		Ok(entry) => {
			let username = &entry[0..entry.find('#').unwrap()];
			let password = &entry[entry.find('#').unwrap() + 1..];
			(username.to_owned(), password.to_owned())
		},
		Err(e) => {
			match e {
				KeyringError::NoPasswordFound => (),
				_ => Err(e).unwrap(),
			}
			let (username, password) = ui.read_auth(domain);
			match key.set_password(&format!("{}#{}", username, password)) {
				Ok(()) | Err(KeyringError::NoBackendFound) => (username, password),
				Err(e) => Err(e).unwrap(),
			}
		},
	}
}
