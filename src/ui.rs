macro_rules! pb_interwrite {
	($pb:expr, $fmt:expr) => {
		{
			use std::io::Write;
			std::io::stdout().write(format!("\r\x1B[K{}\n", $fmt).as_bytes()).unwrap();
			std::io::stdout().flush().unwrap();
			$pb.tick();
		}
	};
	($pb:expr, $fmt:expr $(,$arg:expr)*) => {
		{
			use std;
			use std::io::Write;
			let msg = format!($fmt $(,$arg)*);
			std::io::stdout().write_all(format!("\r\x1B[K{}\n", msg).as_bytes()).unwrap();
			std::io::stdout().flush().unwrap();
			$pb.tick();
		}
	};
}

macro_rules! print_flush {
	($fmt:expr) => {
		{
			use std::io::Write;
			print!($fmt);
			std::io::stdout().flush().unwrap();
		}
	};
	($fmt:expr $(,$arg:expr)*) => {
		{
			use std;
			use std::io::Write;
			print!($fmt $(,$arg)*);
			std::io::stdout().flush().unwrap();
		}
	};
}

use std;

pub fn timefmt(t: std::time::Duration) -> String {
	format!("{}.{:02}s", t.as_secs(), t.subsec_nanos() / 10_000_000)
}
