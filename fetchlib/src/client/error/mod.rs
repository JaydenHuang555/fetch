pub mod block;

pub use crate::client::error::block::BlockedType;
use crate::remote_file_system::{self, error::ExitCode};

#[derive(Debug)]
pub enum ErrorKind {
    RemoteFileSystem(remote_file_system::error::Error),
    LocalFileSystem(std::io::Error),
    Unauthenticated,
    Connection(std::io::Error),
    Blocked(BlockedType, bool),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RemoteFileSystem(e) => write!(f, "remote: {}", e),
            Self::LocalFileSystem(e) => write!(f, "local: {}", e),
            Self::Unauthenticated => write!(f, "Unable to authenticate"),
            Self::Connection(e) => write!(f, "Error in connection to stream: {}", e),
            Self::Blocked(b, over) => {
                write!(
                    f,
                    " {} blocked: {}",
                    if *over {
                        "overridable"
                    } else {
                        "unoverrideable"
                    },
                    b
                )
            }
        }
    }
}

impl From<remote_file_system::error::Error> for ErrorKind {
    fn from(value: remote_file_system::error::Error) -> Self {
        Self::RemoteFileSystem(value)
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub display: Option<&'static str>,
}

impl Error {
    pub fn new(kind: ErrorKind, display: Option<&'static str>) -> Self {
        Self { kind, display }
    }

    pub fn remote_fs(e: remote_file_system::error::Error, display: Option<&'static str>) -> Self {
        Self {
            kind: ErrorKind::RemoteFileSystem(e),
            display,
        }
    }

    pub fn remote_io(e: std::io::Error, code: ExitCode, display: Option<&'static str>) -> Self {
        Self {
            kind: ErrorKind::RemoteFileSystem(remote_file_system::Error::new(code, e.to_string())),
            display,
        }
    }

    pub fn remote_ssh2(e: ssh2::Error, display: Option<&'static str>) -> Self {
        Self::new(
            ErrorKind::RemoteFileSystem(remote_file_system::Error::from(e)),
            display,
        )
    }

    pub fn local_fs(e: std::io::Error, display: Option<&'static str>) -> Self {
        Self {
            kind: ErrorKind::LocalFileSystem(e),
            display,
        }
    }

    pub fn unathenticated(display: Option<&'static str>) -> Self {
        Self {
            kind: ErrorKind::Unauthenticated,
            display,
        }
    }

    pub fn connection(e: std::io::Error, display: Option<&'static str>) -> Self {
        Self {
            kind: ErrorKind::Connection(e),
            display,
        }
    }

    pub fn blocked(bt: BlockedType, can_override: bool, display: Option<&'static str>) -> Self {
        Self {
            kind: ErrorKind::Blocked(bt, can_override),
            display,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason = self.kind.to_string();
        if let Some(msg) = self.display {
            return write!(f, "({}): {}", msg, reason);
        } else {
            return write!(f, "{}", reason);
        }
    }
}

impl std::error::Error for Error {}
