use commands::download::Site;
use reqwest::Url;
use sio2;
use ui::Ui;
use util::sio2_get_session;

pub fn connect(url: &Url, ui: &Ui) -> Box<Site> {
	Box::new(Sio2 { sess: sio2_get_session(url, ui) })
}

struct Sio2 {
	sess: sio2::Session,
}

impl Site for Sio2 {
	fn fetch_resource(&mut self, id: &str) -> Vec<u8> {
		self.sess.get_url(&id.parse().unwrap()) // TODO maybe verify id
	}
}
