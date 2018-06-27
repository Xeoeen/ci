use checkers::Checker;
use diagnose::diagnose_app;
use error::*;
use fitness::Fitness;
use std::{
	borrow::Borrow,
	path::{Path, PathBuf},
};
use strres::{exec, StrRes};
use term_painter::{
	Color::{Green, Red, Yellow},
	ToStyle,
};
use testing::{test_single, TestResult};
use ui::timefmt;
use util::timefn;

pub fn run(gen: &Path, executables: &[PathBuf], checker: &Checker, count: Option<i64>, fitness: &Fitness) -> R<()> {
	for executable in executables.iter() {
		diagnose_app(executable)?;
	}
	let mut i = 1;
	let mut best: Option<(StrRes, i64)> = None;
	while count.map(|n| i <= n).unwrap_or(true) {
		let test_str = exec(&gen, StrRes::Empty).context("failed to run test generator")?;
		print_flush!("(autogenerated {:>6})", i);

		let (out1, t1) = timefn(|| exec(Path::new(&executables[0]), test_str.clone()));

		let out1 = if let Ok(out1) = out1 {
			print_flush!(" {}", Green.bold().paint(timefmt(t1)));
			Some(out1)
		} else {
			print_flush!(" {}", Red.bold().paint(timefmt(t1)));
			None
		};

		let mut all_succeded = out1.is_some();
		for execi in executables[1..].iter() {
			if let Some(perfout) = out1.as_ref() {
				let (outi, ti) = test_single(execi, test_str.clone(), perfout.clone(), checker.borrow())?;
				let rawmsg = timefmt(ti);
				let msg = outi.apply_color(&rawmsg);
				print_flush!(" {}", msg);

				if outi != TestResult::Accept {
					all_succeded = false;
				}
			} else {
				print_flush!(" {}", Yellow.bold().paint("-.--s"));
			}
		}
		println!();
		if !all_succeded {
			if count.is_none() {
				test_str.print_to_stdout();
				break;
			} else {
				let fit = fitness.fitness(test_str.clone())?;
				if best.as_ref().map(|&(_, bfit)| fit > bfit).unwrap_or(true) {
					best = Some((test_str, fit));
				}
			}
		}
		i += 1;
	}

	if count.is_some() {
		if let Some((test_str, _)) = best {
			test_str.print_to_stdout();
		}
	}

	Ok(())
}
