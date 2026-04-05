use std::fmt::Display;
use std::fmt::{self, Formatter};

pub use crate::remote_file_system::exit_code::ExitCode;

#[derive(Debug)]
pub enum EndPoint {
    Local,
    Remote,
}

#[derive(Debug)]
pub struct Error {
    pub code: ExitCode,
    pub display: String,
}

impl Error {
    pub fn new(code: ExitCode, display: String) -> Self {
        Self { code, display }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl From<ssh2::Error> for Error {
    fn from(value: ssh2::Error) -> Self {
        Self {
            code: ExitCode::from(value.code()),
            display: value.to_string(),
        }
    }
}
