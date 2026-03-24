use std::path::PathBuf;

use crate::metadata::FileMetaData;

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
}
