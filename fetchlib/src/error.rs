use crate::remote_file_system::{self, error::ExitCode};

#[derive(Debug)]
pub enum ErrorKind {
    RemoteFileSystem(remote_file_system::error::Error),
    LocalFileSystem(std::io::Error),
    Unauthenticated,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RemoteFileSystem(e) => write!(f, "remote: {}", e),
            Self::LocalFileSystem(e) => write!(f, "local: {}", e),
            Self::Unauthenticated => write!(f, "Unable to authenticate"),
        }
    }
}

impl From<remote_file_system::error::Error> for ErrorKind {
    fn from(value: remote_file_system::error::Error) -> Self {
        Self::RemoteFileSystem(value)
    }
}

impl From<std::io::Error> for ErrorKind {
    fn from(value: std::io::Error) -> Self {
        Self::LocalFileSystem(value)
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub display: Option<&'static str>,
}

macro_rules! new_error {
    ($held_internal:expr, $display_opt:expr) => {
        Self {
            kind: ErrorKind::from($held_internal),
            display: $display_opt,
        }
    };
}

impl Error {
    pub fn new(kind: ErrorKind, display: Option<&'static str>) -> Self {
        Self { kind, display }
    }

    pub fn remote_fs(e: remote_file_system::error::Error, display: Option<&'static str>) -> Self {
        new_error!(e, display)
    }

    pub fn remote_io(e: ExitCode, display: Option<&'static str>) -> Self {
        Self::remote_fs(remote_file_system::Error::new(e, "".to_string()), display)
    }

    pub fn remote_ssh2(e: ssh2::Error, display: Option<&'static str>) -> Self {
        Self::new(
            ErrorKind::RemoteFileSystem(remote_file_system::Error::from(e)),
            display,
        )
    }

    pub fn local_fs(e: std::io::Error, display: Option<&'static str>) -> Self {
        new_error!(e, display)
    }

    pub fn unathenticated(display: Option<&'static str>) -> Self {
        Self {
            kind: ErrorKind::Unauthenticated,
            display,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = self.display {
            return write!(f, "{}", msg);
        }
        self.kind.fmt(f)
    }
}

impl std::error::Error for Error {}
