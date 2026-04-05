use std::str::FromStr;

use crate::fs::FileMetaData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FileSortType {
    FirstCreated,
    LastCreated,
    LastModified,
}

impl FromStr for FileSortType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "first-created" => Ok(Self::FirstCreated),
            "last-modified" => Ok(Self::LastModified),
            "last-created" => Ok(Self::LastCreated),
            _ => Err(String::from("Unkown format found")),
        }
    }
}

impl ToString for FileSortType {
    fn to_string(&self) -> String {
        match self {
            Self::FirstCreated => "first-created",
            Self::LastCreated => "last-created",
            Self::LastModified => "last-modified",
        }
        .to_string()
    }
}

impl FileSortType {
    pub fn sort_vector(&self, files: &mut Vec<FileMetaData>) {
        match self {
            Self::FirstCreated => {
                files.sort_by(|a, b| a.mtime.unwrap().cmp(&b.mtime.unwrap()));
            }
            Self::LastModified => {
                files.sort_by(|a, b| b.mtime.unwrap().cmp(&a.mtime.unwrap()));
            }
            Self::LastCreated => {
                files.sort_by(|a, b| b.atime.unwrap().cmp(&a.atime.unwrap()));
            }
        }
    }
}
