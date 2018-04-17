use std;
use std::fs::File;
use std::io::Write;

pub fn timefn<T, F: FnOnce() -> T>(f: F) -> (T, std::time::Duration) {
	let inst = std::time::Instant::now();
	let x = f();
	let t = inst.elapsed();
	(x, t)
}
pub fn writefile(path: &str, content: &str) {
	let mut f = File::open(path).unwrap();
	f.write_all(content.as_bytes()).unwrap();
}