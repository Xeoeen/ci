use super::{Site, Test};
use colored::*;
use keyring::{Keyring, KeyringError};
use rpassword;
use sio2::{self, Url};
use std::{
	io::{stderr, stdin, Write}, process::{Command, Stdio}, str::from_utf8
};
use tempfile::NamedTempFile;

pub struct Oioioi;

impl Site for Oioioi {
	fn download_tests(url: &Url) -> Vec<Test> {
		let domain = url.domain().unwrap();
		let (user, pass) = read_auth(domain);
		let site = format!("https://{}", domain).parse().unwrap();
		let mut site = sio2::Session::new_login(site, &user, &pass);
		let data = site.get_url(&url);
		let txt = pdf2txt(data);
		parse_siopdf(&txt)
	}
}

fn parse_siopdf(txt: &str) -> Vec<Test> {
	let important_part = &txt[txt.find("Wejście\n\nWyjście\n\n").unwrap()..txt.rfind("\n\n1/1\n").unwrap()];
	let sections: Vec<&str> = important_part.split("\n\n").collect();
	let set_count = sections.iter().filter(|ln| ln.clone() == &"Wejście").count();
	assert_eq!(sections.len(), 4 * set_count);
	(0..set_count)
		.map(|i| Test {
			input: sections[3 * set_count + i].to_owned() + "\n",
			output: sections[3 * i + 2].to_owned() + "\n",
		})
		.collect()
}

fn pdf2txt(data: Vec<u8>) -> String {
	let mut file = NamedTempFile::new().unwrap();
	file.write_all(&data).unwrap();
	let kid = Command::new("pdf2txt.py").stdin(Stdio::piped()).stdout(Stdio::piped()).arg(file.path()).spawn().unwrap();
	let out = kid.wait_with_output().unwrap();
	assert!(out.status.success());
	let txt = from_utf8(&out.stdout).unwrap();
	txt.to_owned()
}

fn read_auth(domain: &str) -> (String, String) {
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
