use commands::list_resources::{Resource, Site};
use reqwest::Url;
use sio2;
use ui::Ui;
use util::sio2_get_session;

pub fn connect(url: &Url, ui: &Ui) -> Box<Site> {
	let sio2::task_url::Deconstructed { contest, .. } = sio2::task_url::deconstruct(&url);
	Box::new(Sio2 {
		sess: sio2_get_session(url, ui),
		contest_name: contest,
	})
}

struct Sio2 {
	sess: sio2::Session,
	contest_name: String,
}

impl Site for Sio2 {
	fn fetch_resource_list(&mut self) -> Vec<Resource> {
		let mut contest = self.sess.contest(&self.contest_name);
		contest
			.files()
			.into_iter()
			.map(|file: sio2::file::File| Resource {
				name: file.name,
				description: file.category.unwrap_or_else(|| String::new()),
				filename: file.filename,
				id: file.link.to_string(),
			})
			.collect()
	}
}
