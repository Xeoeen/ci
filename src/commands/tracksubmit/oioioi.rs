use commands::{
	self, tracksubmit::{Examples, Site}
};
use reqwest::Url;
use sio2::{self, submission::Status, Session};
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
		let examples = match &details.status {
			Status::OK | Status::InitialOK => Some(Examples::OK),
			Status::Pending => None,
			Status::WrongAnswer | Status::TimeLimitExceeded | Status::MemoryLimitExceeded | Status::RuntimeError | Status::CompilationFailed | Status::InitialFailed => {
				Some(Examples::Wrong)
			},
		};
		let score = details.score.clone();
		if let Some(examples) = examples {
			if let Some(score) = score {
				commands::tracksubmit::Status::ScoreReady { examples, score }
			} else {
				commands::tracksubmit::Status::ScorePending { examples }
			}
		} else {
			commands::tracksubmit::Status::InitialPending
		}
	}
}
