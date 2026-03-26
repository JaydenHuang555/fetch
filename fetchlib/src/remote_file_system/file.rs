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
