use error::*;
use std::{
	fs::File,
	io::{Read, Write},
	path::{Path, PathBuf},
	process::{self, Command, Stdio},
	thread::sleep,
	time::{Duration, Instant},
};
use tempfile::NamedTempFile;

pub enum StrRes {
	InMemory(String),
	FileHandle(File),
	FilePath(PathBuf),
	Empty,
}

impl StrRes {
	pub fn from_path(path: &Path) -> StrRes {
		StrRes::FilePath(path.to_owned())
	}

	pub fn get_string(&self) -> R<String> {
		match *self {
			StrRes::InMemory(ref s) => Ok(s.clone()),
			StrRes::FileHandle(ref file) => {
				let mut f2 = file.clone();
				let mut s = String::new();
				f2.read_to_string(&mut s)?;
				Ok(s)
			},
			StrRes::FilePath(ref path) => {
				let file = File::open(path)?;
				StrRes::FileHandle(file).get_string()
			},
			StrRes::Empty => Ok("".to_owned()),
		}
	}

	pub fn with_filename<T, F: FnOnce(&Path) -> T>(&self, f: F) -> T {
		match *self {
			StrRes::FilePath(ref path) => f(path),
			StrRes::InMemory(ref s) => {
				let mut tmp = NamedTempFile::new().unwrap();
				write!(tmp, "{}", s).unwrap();
				f(tmp.path())
			},
			StrRes::Empty => f(Path::new("/dev/null")),
			_ => unimplemented!("StrRes::with_filename"),
		}
	}

	pub fn clone(&self) -> StrRes {
		match *self {
			StrRes::InMemory(ref s) => StrRes::InMemory(s.clone()),
			StrRes::FilePath(ref path) => StrRes::FilePath(path.clone()),
			StrRes::Empty => StrRes::Empty,
			_ => unimplemented!("StrRes::clone"),
		}
	}

	pub fn print_to_stdout(&self) {
		match *self {
			StrRes::InMemory(ref s) => print!("{}", s),
			StrRes::Empty => (),
			_ => unimplemented!("StrRes::print_to_stdout"),
		}
	}
}

#[test]
fn double_strres_getstring_file() {
	const TEST_STR: &str = "Hello, world!\n";
	let f = NamedTempFile::new().unwrap();
	{
		let mut f1 = f.reopen().unwrap();
		f1.write_all(TEST_STR.as_bytes()).unwrap();
		f1.flush().unwrap();
	}
	let s = StrRes::FileHandle(f.reopen().unwrap());
	let r1 = s.get_string().unwrap();
	let r2 = s.get_string().unwrap();
	eprintln!("{:?} -> {:?} {:?}", TEST_STR, r1, r2);
	assert_eq!(r1, TEST_STR);
	assert_eq!(r2, TEST_STR);
}

pub fn exec(executable: &Path, input: StrRes, time_limit: Option<&Duration>) -> R<StrRes> {
	let (stdin_settings, to_write) = match input {
		StrRes::InMemory(s) => (Stdio::piped(), Some(s)),
		StrRes::FileHandle(file) => (Stdio::from(file), None),
		StrRes::FilePath(path) => (Stdio::from(File::open(path).unwrap()), None),
		StrRes::Empty => (Stdio::null(), None),
	};
	let mut kid = Command::new(executable).stdin(stdin_settings).stdout(Stdio::piped()).stderr(Stdio::inherit()).spawn()?;
	if let Some(piped_input) = to_write {
		kid.stdin.as_mut().ok_or(E::StdioFail)?.write_all(piped_input.as_bytes())?;
	}
	let out = match time_limit {
		Some(tl) => {
			let started = Instant::now();
			loop {
				let currt = Instant::now();
				if let Some(status) = kid.try_wait()? {
					let process::Child { stdout, stderr, .. } = kid;
					let mut stdout2 = Vec::new();
					let mut stderr2 = Vec::new();
					if let Some(mut stdos) = stdout {
						stdos.read_to_end(&mut stdout2)?;
					}
					if let Some(mut stdes) = stderr {
						stdes.read_to_end(&mut stderr2)?;
					}
					break process::Output {
						status,
						stdout: stdout2,
						stderr: stderr2,
					};
				}
				if currt - started > *tl {
					return Err(From::from(E::TimeLimitExceeded));
				}
				sleep(Duration::from_micros(10));
			}
		},
		None => kid.wait_with_output()?,
	};

	ensure!(out.status.success(), E::NonZeroStatus(out.status.code().unwrap_or(101)));

	let out_str = String::from_utf8(out.stdout)?;
	Ok(StrRes::InMemory(out_str))
}
