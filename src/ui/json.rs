use super::Ui;
use serde_json;
use std::io::stdin;

pub struct Json;
impl Json {
	pub fn new() -> Json {
		Json
	}
}
impl Ui for Json {
	fn read_auth(&self, domain: &str) -> (String, String) {
		println!("{}", serde_json::to_string(&AuthRequest { domain: domain.to_owned() }).unwrap());
		let mut line = String::new();
		stdin().read_line(&mut line).unwrap();
		let resp: AuthResponse = serde_json::from_str(&line).unwrap();
		(resp.username, resp.password)
	}
}

#[derive(Serialize)]
struct AuthRequest {
	domain: String,
}
#[derive(Deserialize)]
struct AuthResponse {
	username: String,
	password: String,
}
