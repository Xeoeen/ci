use error::*;
use std::{
	fs::{self, File}, io::{self, Read, Write}, path::{Path, PathBuf}, process::{self, Command, Stdio}, str::from_utf8, time::Duration
};
use tempfile::NamedTempFile;
use wait_timeout::ChildExt;

pub enum StrRes {
	InMemory(String),
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
			StrRes::FilePath(ref path) => Ok(from_utf8(&fs::read(path)?)?.to_owned()),
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
		}
	}

	pub fn clone(&self) -> StrRes {
		match *self {
			StrRes::InMemory(ref s) => StrRes::InMemory(s.clone()),
			StrRes::FilePath(ref path) => StrRes::FilePath(path.clone()),
			StrRes::Empty => StrRes::Empty,
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

pub fn exec(executable: &Path, input: StrRes, time_limit: Option<&Duration>) -> R<StrRes> {
	let (stdin_settings, to_write) = match input {
		StrRes::InMemory(s) => (Stdio::piped(), Some(s)),
		StrRes::FilePath(path) => (Stdio::from(File::open(path).unwrap()), None),
		StrRes::Empty => (Stdio::null(), None),
	};
	let mut kid = Command::new(executable).stdin(stdin_settings).stdout(Stdio::piped()).stderr(Stdio::inherit()).spawn()?;
	if let Some(piped_input) = to_write {
		kid.stdin.as_mut().ok_or(E::StdioFail)?.write_all(piped_input.as_bytes())?;
	}
	let (success, excode, stdout) = if let Some(tl) = time_limit {
		if let Some(status) = kid.wait_timeout(tl.clone())? {
			let process::Child { stdout, .. } = kid;
			(status.success(), status.code(), endread(stdout.unwrap())?)
		} else {
			kid.kill()?;
			kid.wait()?;
			return Err(From::from(E::TimeLimitExceeded));
		}
	} else {
		let process::Output { status, stdout, .. } = kid.wait_with_output()?;
		(status.success(), status.code(), stdout)
	};

	ensure!(success, E::NonZeroStatus(excode.unwrap_or(101)));

	let out_str = String::from_utf8(stdout)?;
	Ok(StrRes::InMemory(out_str))
}

pub fn endread<RE: Read>(mut re: RE) -> io::Result<Vec<u8>> {
	let mut buf = Vec::new();
	re.read_to_end(&mut buf)?;
	Ok(buf)
}
