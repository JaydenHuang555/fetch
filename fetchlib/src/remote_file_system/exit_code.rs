use ssh2::ErrorCode;

#[derive(Debug, Clone)]
pub enum ExitCode {
    Session(i32),
    SFTP(i32),
    SCP(i32),
}

impl std::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = match self {
            Self::Session(c) => format!("Session: {}", c),
            Self::SFTP(c) => format!("SFTP: {}", c),
            Self::SCP(c) => format!("SCP: {}", c),
        };
        write!(f, "{}", key)
    }
}

impl From<ErrorCode> for ExitCode {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::Session(c) => ExitCode::Session(c),
            ErrorCode::SFTP(c) => ExitCode::SFTP(c),
        }
    }
}
