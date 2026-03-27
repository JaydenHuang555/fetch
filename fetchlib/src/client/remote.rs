use crate::client::Client;
use crate::remote_file_system::Error;
use crate::remote_file_system::RemoteFileSystem;
use crate::remote_file_system::file::FileMetaData;
use std::path::Path;

impl RemoteFileSystem for Client {
    fn file_metadata(&self, fpath: &Path) -> Result<FileMetaData, Error> {
        let sftp_op = self.session.sftp();
        if let Err(e) = sftp_op {
            return Err(Error::from(e));
        }
        let sftp = sftp_op.unwrap();

        match sftp.stat(fpath) {
            Ok(stat) => {
                let mut meta_data = FileMetaData::from(stat);
                meta_data.path = fpath.to_path_buf();
                Ok(meta_data)
            }
            Err(e) => return Err(Error::from(e)),
        }
    }

    fn listdir(&self, path: &Path) -> Result<Vec<FileMetaData>, crate::remote_file_system::Error> {
        let sftp_op = self.session.sftp();
        if let Err(e) = sftp_op {
            return Err(crate::remote_file_system::Error::from(e));
        }
        let sftp = sftp_op.unwrap();

        match sftp.readdir(path) {
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
