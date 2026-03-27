use std::fmt::Display;

#[derive(Debug)]
pub enum BlockedType {
    FilePresent,
}

impl Display for BlockedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = match self {
            Self::FilePresent => "File already present",
        };
        write!(f, "{}", key)
    }
}
