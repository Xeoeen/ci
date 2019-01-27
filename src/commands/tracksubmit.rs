use error::R;
use std::{thread, time::Duration};
use ui::Ui;
use unijudge;
use util::connect;

pub fn run(url: &str, id: String, sleep_duration: Duration, ui: &mut Ui) -> R<()> {
	let tu = unijudge::TaskUrl::deconstruct(url);
	let sess = connect(url, ui);
	let cont = sess.contest(&tu.contest);
	loop {
		let submissions = cont.submissions_recent();
		let submission = submissions.into_iter().find(|subm| subm.id == id).unwrap();
		ui.track_progress(&submission.verdict);
		let should_end = match &submission.verdict {
			unijudge::Verdict::Pending { .. } => false,
			_ => true,
		};
		if should_end {
			ui.print_finish_tracksubmit(submission.verdict);
			break;
		}
		thread::sleep(sleep_duration);
	}
	Ok(())
}
