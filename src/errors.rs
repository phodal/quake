use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct QuakeError(String);

impl fmt::Display for QuakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for QuakeError {}
