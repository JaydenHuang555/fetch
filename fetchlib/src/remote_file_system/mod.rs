pub mod error;
pub mod exit_code;
pub mod file;

use std::path::{Path, PathBuf};

use crate::{metadata::FileMetaData, remote_file_system::file::FileType};

pub use crate::remote_file_system::error::Error;

pub trait RemoteFileSystem {
    fn file_metadata(&self, fpath: PathBuf) -> FileMetaData;

    fn files_metadata(&self, fpaths: Vec<PathBuf>) -> Vec<FileMetaData> {
        let mut meta_data_list = Vec::new();
        for fpath in fpaths {
            let meta_data = self.file_metadata(fpath);
            meta_data_list.push(meta_data);
        }
        meta_data_list
    }

    fn listdir(&self, path: PathBuf) -> Vec<FileMetaData>;

    fn path_exists(&self, path: PathBuf) -> bool;

    fn dirsize(&self, path: PathBuf) -> Option<u64> {
        let meta_data = self.file_metadata(path);
        meta_data.size.clone()
    }

    fn isdir(&self, path: &Path) -> bool {
        let meta_data = self.file_metadata(path.to_path_buf());
        meta_data.ftype == FileType::Directory
    }
}
