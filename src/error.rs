use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(m: &str) -> Error {
        Error { msg: m.to_string() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
