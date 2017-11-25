macro_rules! pb_interwrite {
	($pb:expr, $fmt:expr) => {
		std::io::stdout().write(format!("\r\x1B[K{}\n", $fmt).as_bytes()).unwrap();
		std::io::stdout().flush().unwrap();
		$pb.tick();
	};
	($pb:expr, $fmt:expr $(,$arg:expr)*) => {
		{
			let msg = format!($fmt $(,$arg)*);
			std::io::stdout().write(format!("\r\x1B[K{}\n", msg).as_bytes()).unwrap();
			std::io::stdout().flush().unwrap();
			$pb.tick();
		}
	};
}


macro_rules! print_flush {
	($fmt:expr) => {
		print!($fmt);
		std::io::stdout().flush().unwrap();
	};
	($fmt:expr $(,$arg:expr)*) => {
		print!($fmt $(,$arg)*);
		std::io::stdout().flush().unwrap();
	};
}
