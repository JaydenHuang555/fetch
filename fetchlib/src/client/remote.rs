use crate::client::Client;
use crate::metadata::FileMetaData;
use crate::remote_file_system::Error;
use crate::remote_file_system::RemoteFileSystem;
use std::path::Path;

impl RemoteFileSystem for Client {
    fn read_file_to_vec(&self, path: &Path, destination: &mut Vec<u8>) -> Result<usize, Error> {
        let recv_operation = self.session.scp_recv(path);
        if let Err(e) = recv_operation {
            return Err(Error::remote_ssh2(
                e,
                Some("Failed to open channel to remote file"),
            ));
        }
        let (mut remote_file_channel, _) = recv_operation.unwrap();
        let read_operation = remote_file_channel.read_to_end(destination);
        match read_operation {
            Ok(read_bytes) => {
                if let Some(e) = remote_secure_shell_channel_close!(remote_file_channel) {
                    return Err(Error::remote_ssh2(e, Some("Failed to close remote server")));
                }
                return Ok(read_bytes);
            }
            Err(e) => {
                let code = { if let Some(c) = e.raw_os_error() { c } else { 1 } };
                return Err(Error::remote_io(ExitCode::SCP(code), None));
            }
        }
    }

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
