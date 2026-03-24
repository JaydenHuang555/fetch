use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum SubcommandsError {
    AttemptToSSHInNonSSHMode,
    InvalidProfile,
}

impl Display for SubcommandsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let key = match self {
            Self::AttemptToSSHInNonSSHMode => "Attempt to do ssh when in a non ssh mode",
            Self::InvalidProfile => "Given Profile was Invalid",
        };
        write!(f, "{}", key)
    }
}

impl std::error::Error for SubcommandsError {}
