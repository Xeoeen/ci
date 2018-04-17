use super::{Site, Test};
use keyring::{Keyring, KeyringError};
use rpassword;
use sio2::{Url};
use std::io::{stdin, stderr, Write};
use colored::*;

pub struct Oioioi;

impl Site for Oioioi {
	fn download_tests(url: &str) -> Vec<Test> {
		let (login, password) = read_auth(Url::parse(url).unwrap().domain().unwrap());
		eprintln!("{} {}", login, password);
		unimplemented!()
	}
}

fn read_auth(domain: &str) -> (String, String) {
	let key = Keyring::new("ci", domain);
	match key.get_password() {
		Ok(entry) => {
			let username = &entry[0..entry.find('#').unwrap()];
			let password = &entry[entry.find('#').unwrap()+1..];
			(username.to_owned(), password.to_owned())
		},
		Err(e) => {
			match e {
				KeyringError::NoPasswordFound => (),
				KeyringError::NoBackendFound => {
					eprintln!("{}", "No keyring found, quit using Arch".red().bold());
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

