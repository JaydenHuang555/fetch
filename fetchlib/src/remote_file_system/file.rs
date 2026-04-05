use std::path::PathBuf;

use ssh2::FileStat;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FileSortType {
    FirstCreated,
    LastCreated,
    LastModified,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum FileType {
    NamedPipe,
    CharDevice,
    Directory,
    RegularFile,
    Symlink,
    Socket,
    Other,
}

impl From<ssh2::FileType> for FileType {
    fn from(other: ssh2::FileType) -> FileType {
        match other {
            ssh2::FileType::NamedPipe => Self::NamedPipe,
            ssh2::FileType::Directory => Self::Directory,
            ssh2::FileType::RegularFile => Self::RegularFile,
            ssh2::FileType::Socket => Self::Socket,
            ssh2::FileType::CharDevice => Self::CharDevice,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileMetaData {
    pub path: PathBuf,
    pub size: Option<u64>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub perm: Option<u32>,
    pub atime: Option<u64>,
    pub mtime: Option<u64>,
    pub ftype: FileType,
}

impl From<FileStat> for FileMetaData {
    fn from(stat: FileStat) -> Self {
        let ftype = FileType::from(stat.clone().file_type());
        Self {
            path: PathBuf::default(),
            size: stat.size,
            uid: stat.uid,
            gid: stat.gid,
            perm: stat.perm,
            atime: stat.atime,
            mtime: stat.mtime,
            ftype: ftype,
        }
    }
}
