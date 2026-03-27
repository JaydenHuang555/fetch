use std::fmt::Display;
use std::fmt::{self, Formatter};
use std::process::ExitCode;

use ssh2::ErrorCode;

#[derive(Debug)]
pub enum Error {
    RemoteIOOperation(std::io::Error),
    SecureShellOperation(ErrorCode, String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::RemoteIOOperation(e) => e.fmt(f),
            Self::SecureShellOperation(code, msg) => write!(
                f,
                "Failed with {} to execute remote operation due to {}",
                code, msg
            ),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Self::RemoteIOOperation(io_error)
    }
}

impl From<ssh2::Error> for Error {
    fn from(other: ssh2::Error) -> Error {
        Self::SecureShellOperation(other.code(), other.message().to_string())
    }
}

impl std::error::Error for Error {}
