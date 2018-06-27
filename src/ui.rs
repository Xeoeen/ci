macro_rules! pb_interwrite {
	($pb:expr, $fmt:expr $(,$arg:expr)*) => {
		{
			print!("\r\x1B[K");
			print!($fmt $(,$arg)*);
			println!();
			$pb.tick();
		}
	};
}

macro_rules! print_flush {
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
