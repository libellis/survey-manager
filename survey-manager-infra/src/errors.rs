use snafu::{Snafu, ResultExt, Backtrace, ErrorCompat, ensure};
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    /// DbFailure conveys to the caller that some kind of error happened on the database level.
    #[snafu(display("database failure"))]
    DbFailure {},
}
