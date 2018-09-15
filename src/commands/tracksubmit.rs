use error::R;
use std::{thread, time::Duration};
use ui::Ui;
use unijudge;
use util::connect;

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
	pub compilation: Compilation,
	pub initial: Outcome,
	pub full: Outcome,
}

pub fn run(url: &str, id: String, sleep_duration: Duration, ui: &Ui) -> R<()> {
	let tu = unijudge::TaskUrl::deconstruct(url);
	let sess = connect(url, ui);
	let cont = sess.contest(&tu.contest);
	loop {
		let submissions = cont.submissions_recent();
		let submission = submissions.iter().find(|subm| subm.id == id).unwrap();
		let (compilation, full) = match submission.verdict {
			unijudge::Verdict::Accepted => (Compilation::Success, Outcome::Success),
			unijudge::Verdict::Rejected => (Compilation::Success, Outcome::Failure),
			unijudge::Verdict::CompilationError => (Compilation::Failure, Outcome::Skipped),
			unijudge::Verdict::Score(score) => (Compilation::Success, Outcome::Score(score)),
			unijudge::Verdict::Pending => (Compilation::Pending, Outcome::Waiting),
		};
		let status = Status {
			compilation,
			initial: Outcome::Unsupported,
			full,
		};
		ui.track_progress(&status);
		let should_end = status.compilation != Compilation::Pending && status.full != Outcome::Pending;
		if should_end {
			break;
		}
		thread::sleep(sleep_duration);
	}
	Ok(())
}
