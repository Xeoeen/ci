use super::Ui;
use rpassword;
use std::io::{stderr, stdin, Write};

pub struct Human;
impl Human {
	pub fn new() -> Human {
		Human
	}
}
impl Ui for Human {
	fn read_auth(&self, domain: &str) -> (String, String) {
		eprintln!("Login required to {}", domain);
		eprint!("  Username: ");
		stderr().flush().unwrap();
		let mut username = String::new();
		stdin().read_line(&mut username).unwrap();
		username = username.trim().to_owned();
		let password = rpassword::prompt_password_stderr("  Password: ").unwrap();
		(username, password)
	}
}
