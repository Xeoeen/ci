use checkers::Checker;
use diagnose::*;
use error::*;
use itertools::{self, Itertools};
use std::{self, borrow::Borrow, cmp::Ordering, path::Path};
use strres::StrRes;
use testing::{test_single, TestResult};
use ui::Ui;
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

fn recursive_find_tests(testdir: &Path) -> Box<Iterator<Item=std::path::PathBuf>> {
	let mut tests: Vec<_> = walkdir::WalkDir::new(testdir)
		.follow_links(true)
		.into_iter()
		.filter_map(|e| e.ok())
		.map(|entry| entry.path().to_path_buf())
		.filter(|path| path.extension().map(|ext| ext == "in").unwrap_or(false))
		.collect();
	tests.sort_by(ord_by_test_number);
	Box::new(tests.into_iter())
}

pub fn run(executable: &Path, testdir: &Path, checker: &Checker, no_print_success: bool, print_output: bool, ui: &mut Ui) -> R<bool> {
	ensure!(testdir.exists(), err_msg("test directory does not exist"));
	diagnose_app(&executable, ui)?;
	diagnose_checker(checker, ui)?;
	let mut good = true;
	for in_path in recursive_find_tests(&testdir) {
		let out_path = in_path.with_extension("out");
		let (output, outcome, timing) = if out_path.exists() {
			let (out, outcome, timing) = test_single(&executable, StrRes::from_path(&in_path), StrRes::from_path(&out_path), checker.borrow(), None)?;
			(out, outcome, Some(timing))
		} else {
			(StrRes::Empty, TestResult::IgnoredNoOut, None)
		};
		if outcome != TestResult::Accept || !no_print_success {
			ui.print_test(&outcome, timing, &in_path, if print_output { Some(output) } else { None });
		}
		if outcome != TestResult::Accept {
			good = false;
		}
	}
	ui.print_finish_test(good);
	Ok(good)
}
