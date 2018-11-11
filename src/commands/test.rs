use checkers::Checker;
use diagnose::*;
use error::*;
use itertools::{self, Itertools};
use std::{self, borrow::Borrow, cmp::Ordering, path::Path, sync::Mutex};
use strres::StrRes;
use testing::{test_single, TestResult};
use ui::Ui;
use rayon::prelude::*;
use walkdir;

fn ord_by_test_number(lhs: &std::path::PathBuf, rhs: &std::path::PathBuf) -> Ordering {
	for grp in lhs
		.to_str()
		.unwrap()
		.chars()
		.group_by(|c| c.is_numeric())
		.into_iter()
		.zip_longest(rhs.to_str().unwrap().chars().group_by(|c| c.is_numeric()).into_iter())
	{
		match grp {
			itertools::EitherOrBoth::Both((isdig, lgrp), (_, rgrp)) => {
				let grp_compr = if isdig {
					let lnum: i64 = lgrp.collect::<String>().parse().unwrap();
					let rnum: i64 = rgrp.collect::<String>().parse().unwrap();
					lnum.cmp(&rnum)
				} else {
					lgrp.cmp(rgrp)
				};
				if grp_compr != Ordering::Equal {
					return grp_compr;
				}
			},
			itertools::EitherOrBoth::Left(_) => return Ordering::Greater,
			itertools::EitherOrBoth::Right(_) => return Ordering::Less,
		}
	}
	Ordering::Equal
}


fn recursive_find_tests(testdir: &Path) -> Vec<std::path::PathBuf> {
	let mut tests: Vec<_> = walkdir::WalkDir::new(testdir)
		.follow_links(true)
		.into_iter()
		.filter_map(|e| e.ok())
		.map(|entry| entry.path().to_path_buf())
		.filter(|path| path.extension().map(|ext| ext == "in").unwrap_or(false))
		.collect();
	tests.sort_by(ord_by_test_number);
	tests
}


pub fn run(executable: &Path, testdir: &Path, checker: &Checker, no_print_success: bool, print_output: bool, ui: &mut Ui) -> R<()> {
	ensure!(testdir.exists(), err_msg("test directory does not exist"));
	diagnose_app(&executable, ui)?;
	diagnose_checker(checker, ui)?;

	let shared_ui = Mutex::new(ui);
	let tests = recursive_find_tests(&testdir);
	for (key, group) in tests.iter().group_by(|in_path| in_path.parent().unwrap()).into_iter() {
		{
			let mut ui = shared_ui.lock().unwrap();
			ui.notice(&format!("Entering directory {}", key.to_str().unwrap()));
		}
		let test_group = group.into_iter().collect_vec();
		let results: Vec<_> = test_group.par_iter().map(|in_path|{

			let out_path = in_path.with_extension("out");
			let (output, outcome, timing) = if out_path.exists() {
				match test_single(&executable, StrRes::from_path(&in_path), StrRes::from_path(&out_path), checker.borrow(), None) {
					Ok((out, outcome, timing)) => (out, outcome, Some(timing)),
					Err(err) => return Err(err)
				}
			} else {
				(StrRes::Empty, TestResult::IgnoredNoOut, None)
			};
			if outcome != TestResult::Accept || !no_print_success {
				let mut ui= shared_ui.lock().unwrap();
				ui.print_test(&outcome, timing, &in_path, if print_output { Some(output) } else { None });
			}

			Ok(outcome == TestResult::Accept)
		}).collect();

		for res in results.into_iter() {
			match res {
				Ok(good) => if !good { std::process::exit(1); }
				Err(err) => return Err(err)
			}
		}
	}


	Ok(())
}
