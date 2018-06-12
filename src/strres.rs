use error::*;
use std::{
	fs::File,
	io::{Read, Write},
	path::{Path, PathBuf},
	process::{Command, Stdio},
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
		match self {
			&StrRes::InMemory(ref s) => Ok(s.clone()),
			&StrRes::FileHandle(ref file) => {
				let mut f2 = file.clone();
				let mut s = String::new();
				f2.read_to_string(&mut s)?;
				Ok(s)
			},
			&StrRes::FilePath(ref path) => {
				let file = File::open(path)?;
				StrRes::FileHandle(file).get_string()
			},
			&StrRes::Empty => Ok("".to_owned()),
		}
	}

	pub fn with_filename<T, F: FnOnce(&Path) -> T>(&self, f: F) -> T {
		match self {
			&StrRes::FilePath(ref path) => f(path),
			&StrRes::InMemory(ref s) => {
				let mut tmp = NamedTempFile::new().unwrap();
				write!(tmp, "{}", s).unwrap();
				f(tmp.path())
			},
			&StrRes::Empty => f(Path::new("/dev/null")),
			_ => unimplemented!("StrRes::with_filename"),
		}
	}

	pub fn clone(&self) -> StrRes {
		match self {
			&StrRes::InMemory(ref s) => StrRes::InMemory(s.clone()),
			&StrRes::FilePath(ref path) => StrRes::FilePath(path.clone()),
			&StrRes::Empty => StrRes::Empty,
			_ => unimplemented!("StrRes::clone"),
		}
	}

	pub fn print_to_stdout(&self) {
		match self {
			&StrRes::InMemory(ref s) => print!("{}", s),
			&StrRes::Empty => (),
			_ => unimplemented!("StrRes::print_to_stdout"),
		}
	}
}

pub fn exec(executable: &Path, input: StrRes) -> R<StrRes> {
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
	let out = kid.wait_with_output()?;

	ensure!(out.status.success(), E::NonZeroStatus(out.status.code().unwrap_or(101)));

	let out_str = String::from_utf8(out.stdout)?;
	Ok(StrRes::InMemory(out_str))
}
