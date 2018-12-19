#[macro_use]
extern crate structopt;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate itertools;
extern crate keyring;
extern crate rpassword;
extern crate serde;
extern crate serde_json;
extern crate tempfile;
extern crate term;
extern crate unijudge;
extern crate wait_timeout;
extern crate walkdir;

pub mod auth;
pub mod checkers;
pub mod diagnose;
pub mod error;
pub mod strres;
pub mod testing;
#[macro_use]
pub mod ui;
pub mod cli;
pub mod commands;
pub mod fitness;
pub mod util;

pub use cli::{Args, Command};
pub use commands::build::Codegen;
pub use error::*;
pub use std::{borrow::Borrow, ops::DerefMut};
pub use structopt::StructOpt;
pub use ui::Ui;
