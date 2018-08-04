use super::Ui;
use keyring::{Keyring, KeyringError};
use rpassword;
use std::io::{stderr, stdin, Write};
use term_painter::{Color::Red, ToStyle};

pub struct Human;
impl Human {
	pub fn new() -> Human {
		Human
	}
}
impl Ui for Human {
	fn read_auth(&self, domain: &str) -> (String, String) {
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
					KeyringError::NoBackendFound => {
						eprintln!("{}", Red.bold().paint("No keyring found, quit using Arch"));
					},
					_ => Err(e).unwrap(),
				}
				eprintln!("Login required to {}", domain);
				eprint!("  Username: ");
				stderr().flush().unwrap();
				let mut username = String::new();
				stdin().read_line(&mut username).unwrap();
				username = username.trim().to_owned();
				let password = rpassword::prompt_password_stderr("  Password: ").unwrap();
				match key.set_password(&format!("{}#{}", username, password)) {
					Ok(()) | Err(KeyringError::NoBackendFound) => (username, password),
					Err(e) => Err(e).unwrap(),
				}
			},
		}
	}
}
