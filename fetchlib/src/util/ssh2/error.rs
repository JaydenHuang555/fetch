use std::fmt::Display;

#[derive(Debug)]
pub enum ErrorKind {
    LocalIO(std::io::Error),
    RemoteIO(std::io::Error),
    SSH(ssh2::Error),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalIO(e) | Self::RemoteIO(e) => write!(
                f,
                "{} IO operation error: {}",
                {
                    if let Self::LocalIO(_) = self {
                        "Local"
                    } else {
                        "Remote"
                    }
                },
                e
            ),
            Self::SSH(e) => write!(f, "ssh error: {}", e),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    display: Option<&'static str>,
}

impl Error {
    pub fn new(kind: ErrorKind, display: Option<&'static str>) -> Self {
        Self { kind, display }
    }

    pub fn local_io(e: std::io::Error, display: Option<&'static str>) -> Self {
        Self {
            kind: ErrorKind::LocalIO((e)),
            display,
        }
    }

    pub fn remote_io(e: std::io::Error, display: Option<&'static str>) -> Self {
        Self {
            kind: ErrorKind::RemoteIO(e),
            display,
        }
    }

    pub fn ssh(e: ssh2::Error, display: Option<&'static str>) -> Self {
        Self {
            kind: ErrorKind::SSH(e),
            display,
        }
    }

    pub fn display(&self) -> Option<&'static str> {
        self.display
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = self.kind.to_string();
        if let Some(display) = self.display() {
            write!(f, "({}): {}", display, kind)
        } else {
            write!(f, "{}", kind)
        }
    }
}

impl std::error::Error for Error {}
