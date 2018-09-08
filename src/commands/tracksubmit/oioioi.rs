use commands::{
	self, tracksubmit::{Compilation, Outcome, Site, Status}
};
use reqwest::Url;
use sio2::{self, Session};
use ui::Ui;
use util::sio2_get_session;

pub fn connect(url: &Url, id: &str, ui: &Ui) -> Box<Site> {
	let sio2::task_url::Deconstructed { contest, .. } = sio2::task_url::deconstruct(&url);
	let sess = sio2_get_session(url, ui);
	Box::new(Oioioi {
		sess,
		contest_name: contest.to_owned(),
		subm_id: id.parse().unwrap(),
	})
}

struct Oioioi {
	sess: Session,
	contest_name: String,
	subm_id: i64,
}

impl Site for Oioioi {
	fn fetch_status(&mut self) -> commands::tracksubmit::Status {
		let mut contest = self.sess.contest(&self.contest_name);
		let details = contest.submission_details(self.subm_id);
		let full = if let Some(score) = details.score {
			Outcome::Score(score)
		} else {
			match &details.status {
				sio2::Status::CompilationFailed => Outcome::Skipped,
				sio2::Status::OK
				| sio2::Status::WrongAnswer
				| sio2::Status::TimeLimitExceeded
				| sio2::Status::MemoryLimitExceeded
				| sio2::Status::RuntimeError
				| sio2::Status::InitialOK
				| sio2::Status::InitialFailed => Outcome::Pending,
				sio2::Status::Pending => Outcome::Waiting,
			}
		};
		match &details.status {
			sio2::Status::CompilationFailed => Status {
				compilation: Compilation::Failure,
				initial: Outcome::Skipped,
				full,
			},
			sio2::Status::Pending => Status {
				compilation: Compilation::Pending,
				initial: Outcome::Waiting,
				full,
			},
			sio2::Status::InitialOK => Status {
				compilation: Compilation::Success,
				initial: Outcome::Success,
				full,
			},
			sio2::Status::InitialFailed => Status {
				compilation: Compilation::Success,
				initial: Outcome::Failure,
				full,
			},
			sio2::Status::OK | sio2::Status::WrongAnswer | sio2::Status::TimeLimitExceeded | sio2::Status::MemoryLimitExceeded | sio2::Status::RuntimeError => Status {
				compilation: Compilation::Success,
				initial: Outcome::Skipped,
				full,
			},
		}
	}
}
