use std::fs::{File};
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub enum StrRes {
	InMemory(String),
	FileHandle(File),
	FilePath(PathBuf),
}

impl StrRes {

	pub fn get_string(self) -> String {
		match self {
			StrRes::InMemory(s) => s,
			StrRes::FileHandle(mut file) => {
				let mut s = String::new();
				file.read_to_string(&mut s).unwrap();
				s
			},
			StrRes::FilePath(path) => {
				let file = File::open(path).unwrap();
				StrRes::FileHandle(file).get_string()
			},
		}
	}
	pub fn with_filename<T, F: FnOnce(&Path) -> T>(&self, f: F) -> T {
		match self {
			&StrRes::FilePath(ref path) => f(path),
			_ => unimplemented!("StrRes::with_filename"),
		}
	}

}

pub fn exec(executable: &Path, input: StrRes) -> StrRes {
	let (stdin_settings, to_write) = match input {
		StrRes::InMemory(s) => (Stdio::piped(), Some(s)),
		StrRes::FileHandle(file) => (Stdio::from(file), None),
		StrRes::FilePath(path) => (Stdio::from(File::open(path).unwrap()), None),
	};
	let mut kid = Command::new(executable)
		.stdin(stdin_settings)
		.stdout(Stdio::piped())
		.stderr(Stdio::inherit())
		.spawn().unwrap();
	if let Some(piped_input) = to_write {
		kid.stdin.as_mut().unwrap().write_all(piped_input.as_bytes()).unwrap();
	}
	let out = kid.wait_with_output().unwrap();
	assert!(out.status.success());
	let out_str = String::from_utf8(out.stdout).unwrap();
	StrRes::InMemory(out_str)
}