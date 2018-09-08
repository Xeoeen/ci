mod codeforces;
mod oioioi;

use error::{E, R};
use failure::Error;
use reqwest::Url;
use std::{thread, time::Duration};
use ui::Ui;

#[derive(Serialize, PartialEq, Eq)]
pub enum Compilation {
	Pending,
	Success,
	Failure,
}
#[derive(Serialize, PartialEq, Eq)]
pub enum Outcome {
	Unsupported,
	Skipped,
	Waiting, // waiting for other things to finish first
	Pending, // this will finish first
	Success,
	Failure,
	Score(i64),
}
#[derive(Serialize)]
pub struct Status {
	compilation: Compilation,
	initial: Outcome,
	full: Outcome,
}

pub trait Site {
	fn fetch_status(&mut self) -> Status;
}

type Connector = fn(&Url, &str, &Ui) -> Box<Site>;
const MATCHERS: &[(&str, Connector)] = &[("sio2.staszic.waw.pl", oioioi::connect), ("codeforces.com", codeforces::connect)];

pub fn run(url: &Url, id: String, sleep_duration: Duration, ui: &Ui) -> R<()> {
	let domain = url.domain().unwrap();
	let connector = MATCHERS
		.iter()
		.find(|&&(dom, _)| dom == domain)
		.ok_or_else(|| Error::from(E::UnsupportedProblemSite(domain.to_owned())))?;
	let mut site = (connector.1)(url, &id, ui);
	loop {
		let status = site.fetch_status();
		ui.track_progress(&status);
		let should_end = status.compilation != Compilation::Pending && status.initial != Outcome::Pending && status.full != Outcome::Pending;
		if should_end {
			break;
		}
		thread::sleep(sleep_duration);
	}
	Ok(())
}
