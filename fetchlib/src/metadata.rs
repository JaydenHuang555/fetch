use std::path::PathBuf;

use ssh2::FileStat;

#[derive(Debug, Clone)]
pub struct FileMetaData {
    pub path: PathBuf,
    pub size: Option<u64>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub perm: Option<u32>,
    pub atime: Option<u64>,
    pub mtime: Option<u64>,
}

impl From<FileStat> for FileMetaData {
    fn from(stat: FileStat) -> Self {
        Self {
            path: PathBuf::default(),
            size: stat.size,
            uid: stat.uid,
            gid: stat.gid,
            perm: stat.perm,
            atime: stat.atime,
            mtime: stat.mtime,
        }
    }
}
