use super::Error;
use crate::{fs::FileMetaData, remote_file_system::RemoteFileSystem};

pub struct Sftp {
    sftp: ssh2::Sftp,
}

impl RemoteFileSystem for Sftp {
    fn file_metadata(
        &self,
        fpath: &std::path::Path,
    ) -> Result<crate::fs::FileMetaData, super::Error> {
        match self.sftp.stat(fpath) {
            Ok(stat) => {
                let mut meta_data = FileMetaData::from(stat);
                meta_data.path = fpath.to_path_buf();
                return Ok(meta_data);
            }
            Err(e) => Err(super::Error::from(e)),
        }
    }

    fn listdir(
        &self,
        path: &std::path::Path,
    ) -> Result<Vec<crate::fs::FileMetaData>, super::Error> {
        match self.sftp.readdir(path) {
            Ok(contents) => {
                let output: Vec<FileMetaData> = contents
                    .into_iter()
                    .map(|c| {
                        let mut m = FileMetaData::from(c.1);
                        m.path = c.0;
                        m
                    })
                    .collect();
                return Ok(output);
            }
            Err(e) => return Err(Error::from(e)),
        }
    }
}
