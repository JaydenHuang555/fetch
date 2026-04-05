pub mod fs;
pub mod interact;

pub struct Sftp {
    sftp: ssh2::Sftp,
}

impl From<ssh2::Sftp> for Sftp {
    fn from(value: ssh2::Sftp) -> Self {
        Self { sftp: value }
    }
}
