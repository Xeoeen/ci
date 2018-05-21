use checkers::CheckerBox;
use colored::*;
use diagnose::diagnose_app;
use error::*;
use std::path::{Path, PathBuf};
use strres::{exec, StrRes};
use testing::{test_single, TestResult};
use ui::timefmt;
use util::timefn;

pub fn run(gen: &Path, executables: &[PathBuf], checker: CheckerBox, count: Option<i64>) -> R<()> {
	for ref executable in executables {
		diagnose_app(executable)?;
	}
	let mut i = 1;
	let mut best: Option<(StrRes, i64)> = None;
	while count.map(|n| i <= n).unwrap_or(true) {
		let test_str = exec(&gen, StrRes::Empty)?;
		print_flush!("(autogenerated {:>6})", i);

		let (out1, t1) = timefn(|| exec(Path::new(&executables[0]), test_str.clone()));

		let out1 = if let Ok(out1) = out1 {
			print_flush!(" {}", timefmt(t1).green().bold());
			out1
		} else {
			print_flush!(" {}\n", timefmt(t1).red().bold());
			test_str.print_to_stdout();
			break;
		};

		let mut all_succeded = true;
		for ref execi in &executables[1..] {
			let (outi, ti) = test_single(execi, test_str.clone(), out1.clone(), checker.as_ref())?;
			let msg = outi.apply_color(&timefmt(ti));
			print_flush!(" {}", msg);

			if outi != TestResult::Accept {
				all_succeded = false;
			}
		}
		println!("");
		if !all_succeded {
			if count.is_none() {
				test_str.print_to_stdout();
				break;
			} else {
				let fit = fitness(&test_str);
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

fn fitness(res: &StrRes) -> i64 {
	let s = res.get_string().unwrap();
	-(s.len() as i64)
}
