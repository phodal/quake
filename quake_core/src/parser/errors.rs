use std::{fmt, io};
use std::error::Error as StdError;
use std::fmt::{Debug, Formatter};

pub struct QuakeParserError {
    msg: String,
}

impl QuakeParserError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: String::from(msg),
        }
    }
}

impl fmt::Display for QuakeParserError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "QuakeParserError: {}", self.msg)
    }
}

impl Debug for QuakeParserError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "QuakeParserError: {}", self.msg)
    }
}

impl StdError for QuakeParserError {

}

impl From<io::Error> for QuakeParserError {
    fn from(err: io::Error) -> Self {
        Self {
            msg: format!("cause by: {}", err),
        }
    }
}
