pub use failure::{err_msg, Error, Fail, ResultExt};

pub type R<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum E {
	#[fail(display = "invalid exit status code: {}", _0)]
	NonZeroStatus(i32),
	#[fail(display = "could not open stdin")]
	StdioFail,
	#[fail(display = "expected file {} to have .{} extension", _1, _0)]
	InvalidFileExtension(String, String),
	#[fail(display = "unsupported problem site {}", _0)]
	UnsupportedProblemSite(String),
	#[fail(display = "time limit exceeded")]
	TimeLimitExceeded,
}

#[derive(Debug, Fail)]
#[fail(display = "failed to parse cli arguments, expected {}, got {}", _0, _1)]
pub struct ParseError {
	pub expected: &'static str,
	pub found: String,
}
