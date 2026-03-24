use std::{
    error::Error,
    fmt::{self, Display},
    io,
};

#[derive(Debug)]
pub enum CollisionType {
    ProfileKey(String),
    FileName(String),
}

impl Display for CollisionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ProfileKey(key) => write!(f, "profile key ({})", key),
            Self::FileName(name) => write!(f, "file name {}", name),
        }
    }
}

#[derive(Debug)]
pub enum ProfileManagerError {
    DirectoryIO(io::Error),
    FileIO(io::Error),
    InvalidDirectory(&'static str),
    CollisionDetected(CollisionType),
    SerializeError(serde_json::error::Error),
    DeserializeError(serde_json::error::Error),
}

impl Display for ProfileManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DirectoryIO(e) | Self::FileIO(e) => e.fmt(f),
            Self::InvalidDirectory(message) => write!(f, "Invalid Directory: Required {}", message),
            Self::CollisionDetected(key) => write!(f, "Collision Detected: {}", key),
            Self::SerializeError(e) => e.fmt(f),
            Self::DeserializeError(e) => e.fmt(f),
        }
    }
}

impl Error for ProfileManagerError {}

#[derive(Debug)]
pub enum ProfileError {
    FileIO(io::Error),
    SerializeErr(serde_json::error::Error),
    DeserializeErr(serde_json::error::Error),
}

impl Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileIO(e) => e.fmt(f),
            Self::SerializeErr(e) => e.fmt(f),
            Self::DeserializeErr(e) => e.fmt(f),
        }
    }
}

impl Error for ProfileError {}
