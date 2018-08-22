mod oioioi;

use error::{E, R};
use failure::Error;
use reqwest::Url;
use std::{thread, time::Duration};
use ui::Ui;

pub enum Examples {
	OK,
	Wrong,
}
pub enum Status {
	InitialPending,
	// 	RevealPending { examples: Examples },
	// 	RevealReady { examples: Examples },
	// TODO reveals
	ScorePending { examples: Examples },
	ScoreReady { examples: Examples, score: i64 },
}

pub trait Site {
	fn fetch_status(&mut self) -> Status;
}

type Connector = fn(&Url, &str, &Ui) -> Box<Site>;
const MATCHERS: &[(&str, Connector)] = &[("sio2.staszic.waw.pl", oioioi::connect)];

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
		let should_end = match status {
			Status::InitialPending{..} | Status::ScorePending{..} /*| Status::RevealPending{..}*/ => false,
			/*Status::RevealReady{..} |*/ Status::ScoreReady{..} => true,
		};
		if should_end {
			break;
		}
		thread::sleep(sleep_duration);
	}
	Ok(())
}
