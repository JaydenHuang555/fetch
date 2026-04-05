use crate::{remote_interact::RemoteTransferProtocol, sftp::Sftp, util::ssh2::Error};

use std::io::{Read, Write};
use std::path::Path;

impl RemoteTransferProtocol for Sftp {
    type Error = Error;
    fn read_to_file(&self, source: &Path, destination: &Path) -> Result<usize, Error> {
        let open_operation = self.sftp.open(source);
        if let Err(e) = open_operation {
            return Err(Error::ssh(e, Some("Failed to open source file")));
        }

        let file_creation = std::fs::File::create(destination);

        if let Err(e) = file_creation {
            return Err(Error::local_io(
                e,
                Some("Failed to create destination file"),
            ));
        }

        let mut destination_file = file_creation.unwrap();

        let mut source_file = open_operation.unwrap();
        let mut chunk = [0u8; 512];
        let mut read_bytes = 0;
        loop {
            match source_file.read(&mut chunk) {
                Ok(read_chunk_bytes) => {
                    if read_chunk_bytes == 0 {
                        break;
                    }
                    if let Err(e) = destination_file.write_all(&mut chunk) {
                        return Err(Error::local_io(
                            e,
                            Some("Failed to write read contents to destination"),
                        ));
                    }
                    read_bytes = read_bytes + read_chunk_bytes;
                }
                Err(e) => {
                    return Err(Error::remote_io(e, Some("Failed to read source to chunk")));
                }
            }
        }
        Ok(read_bytes)
    }
}
