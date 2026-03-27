use clap::ValueEnum;
use fetchlib::remote_file_system::file::FileMetaData;

#[derive(Clone, ValueEnum, Debug)]
pub enum SortMode {
    LastCreated,
    FirstCreated,
}

impl ToString for SortMode {
    fn to_string(&self) -> String {
        match self {
            Self::LastCreated => "last-created",
            Self::FirstCreated => "first-created",
        }
        .to_string()
    }
}

impl SortMode {
    pub fn sort(&self, files: &mut Vec<FileMetaData>) {
        match self {
            Self::LastCreated => {
                files.sort_by(|a, b| b.mtime.unwrap().cmp(&a.mtime.unwrap()));
            }
            Self::FirstCreated => {
                files.sort_by(|a, b| a.mtime.unwrap().cmp(&b.mtime.unwrap()));
            }
        }
    }
}
