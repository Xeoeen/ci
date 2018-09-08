use auth;
use codeforces::{self, Verdict};
use commands::tracksubmit::{Compilation, Outcome, Site, Status};
use reqwest::Url;
use ui::Ui;

pub fn connect(url: &Url, id: &str, ui: &Ui) -> Box<Site> {
	let (user, pass) = auth::get("codeforces.com", ui);
	let mut sess = codeforces::Session::new().expect("failed to create Codeforces session");
	sess.login(&user, &pass).expect("failed to login to Codeforces");
	Box::new(Codeforces {
		sess,
		task_url: url.clone(),
		submission_id: id.parse().expect("cf submit id not a number"),
	})
}

struct Codeforces {
	sess: codeforces::Session,
	task_url: Url,
	submission_id: i64,
}

impl Site for Codeforces {
	fn fetch_status(&mut self) -> Status {
		let task_url = self.task_url.clone();
		let submission_id = self.submission_id;
		let mut prob = self.sess.problem_from_url(&task_url).expect("failed to get cf problem");
		let submission = prob
			.contest_submissions()
			.expect("failed to fetch submissions page")
			.into_iter()
			.find(|submission| submission.id == submission_id)
			.expect("submission with given id not found");
		match submission.verdict {
			Verdict::CompilationError => Status {
				compilation: Compilation::Failure,
				initial: Outcome::Unsupported,
				full: Outcome::Skipped,
			},
			Verdict::Accepted => Status {
				compilation: Compilation::Success,
				initial: Outcome::Unsupported,
				full: Outcome::Success,
			},
			Verdict::WrongAnswer(_) | Verdict::RuntimeError(_) | Verdict::MemoryLimitExceeded(_) | Verdict::TimeLimitExceeded(_) | Verdict::Hacked => Status {
				compilation: Compilation::Success,
				initial: Outcome::Unsupported,
				full: Outcome::Failure,
			},
			Verdict::Testing(_) => Status {
				compilation: Compilation::Success,
				initial: Outcome::Unsupported,
				full: Outcome::Pending,
			},
		}
	}
}
