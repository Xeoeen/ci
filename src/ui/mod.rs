macro_rules! pb_interwrite {
	($pb:expr, $fmt:expr $(,$arg:expr)*) => {
		{
			eprint!("\r\x1B[K");
			eprint!($fmt $(,$arg)*);
			eprintln!();
			$pb.tick();
		}
	};
}

macro_rules! eprint_flush {
	($fmt:expr $(,$arg:expr)*) => {
		{
			use std;
			use std::io::Write;
			eprint!($fmt $(,$arg)*);
			std::io::stderr().flush().unwrap();
		}
	};
}

mod human;
mod json;

use std;

pub fn timefmt(t: std::time::Duration) -> String {
	format!("{}.{:02}s", t.as_secs(), t.subsec_nanos() / 10_000_000)
}

pub trait Ui {
	fn read_auth(&self, domain: &str) -> (String, String);
}

pub use self::{human::Human, json::Json};
