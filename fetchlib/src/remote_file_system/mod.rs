pub mod error;
pub mod exit_code;

use std::path::Path;

use crate::fs::{FileMetaData, FileType, sort::FileSortType};

pub use crate::remote_file_system::error::Error;

pub trait RemoteFileSystem {
    fn file_metadata(&self, fpath: &Path) -> Result<FileMetaData, Error>;

    fn files_metadata(&self, fpaths: Vec<&Path>) -> Result<Vec<FileMetaData>, Error> {
        let mut meta_data_list = Vec::new();
        for fpath in fpaths {
            match self.file_metadata(fpath) {
                Ok(meta_data) => meta_data_list.push(meta_data),
                Err(e) => return Err(e),
            }
        }
        Ok(meta_data_list)
    }

    fn listdir(&self, path: &Path) -> Result<Vec<FileMetaData>, Error>;

    #[deprecated(
        since = "0.0.13",
        note = "please use `disort_files` instead with FileSortType::LastModified as the passed in FileSortType"
    )]
    fn last_mod_file(&self, path: &Path) -> Result<FileMetaData, Error> {
        match self.listdir(path) {
            Ok(mut meta_datas) => {
                meta_datas.sort_by(|a, b| {
                    b.mtime
                        .unwrap_or_default()
                        .cmp(&a.mtime.unwrap_or_default())
                });
                Ok(meta_datas[0].clone())
            }
            Err(e) => Err(e),
        }
    }

    fn dirsort_file(&self, path: &Path, sort: FileSortType) -> Result<FileMetaData, Error> {
        let read = self.listdir(path);
        if let Err(e) = read {
            return Err(e);
        }

        let mut files = read.unwrap();

        sort.sort_vector(&mut files);

        Ok(files[0].clone())
    }

    fn path_exists(&self, path: &Path) -> bool {
        self.file_metadata(path).is_ok()
    }

    fn dirsize(&self, path: &Path) -> Result<Option<u64>, Error> {
        match self.file_metadata(path) {
            Ok(meta_data) => Ok(meta_data.size.clone()),
            Err(e) => Err(e),
        }
    }

    fn isdir(&self, path: &Path) -> bool {
        match self.file_metadata(path) {
            Ok(meta_data) => meta_data.ftype == FileType::Directory,
            Err(_) => false,
        }
    }

    fn isfile(&self, path: &Path) -> bool {
        match self.file_metadata(path) {
            Ok(meta_data) => meta_data.ftype == FileType::RegularFile,
            Err(_) => false,
        }
    }
}
