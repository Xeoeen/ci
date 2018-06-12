use std::{
	self,
	fs::{create_dir, File},
	io::{self, Write},
};

pub fn timefn<T, F: FnOnce() -> T>(f: F) -> (T, std::time::Duration) {
	let inst = std::time::Instant::now();
	let x = f();
	let t = inst.elapsed();
	(x, t)
}
pub fn writefile(path: &str, content: &str) {
	let mut f = File::create(path).unwrap();
	f.write_all(content.as_bytes()).unwrap();
}
pub fn demand_dir(path: &str) -> Result<(), io::Error> {
	match create_dir(path) {
		Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(()),
		r @ _ => r,
	}
}
