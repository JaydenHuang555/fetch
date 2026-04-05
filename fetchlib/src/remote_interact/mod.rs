use std::path::Path;

pub trait RemoteTransferProtocol {
    type Error: std::error::Error;

    fn read_to_file(&self, source: &Path, destination: &Path) -> Result<usize, Self::Error>;
}
