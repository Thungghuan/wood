use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: String) -> Self {
        Error { msg }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Error: {{\n  msg: {}\n}}", &self.msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Error: {{\n  msg: {}\n}}", &self.msg)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error {
            msg: format!("{}", err),
        }
    }
}
