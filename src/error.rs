pub use failure::{
    Fail,
    ResultExt,
    Error,
    err_msg
};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum RuntimeError {
     #[fail(display = "invalid exit status code: {}", _0)]
	NonZeroStatus(i32),
    #[fail(display = "could not open stdin")]
    StdioFail
}

#[derive(Debug, Fail)]
pub enum FileError {
     #[fail(display = "expected extension {} for file: {}", _0, _1)]
	InvalidFileExtension(String, String),
}

#[derive(Debug, Fail)]
#[fail(display = "failed while parsing cli arguments")]
pub struct ParseError;
