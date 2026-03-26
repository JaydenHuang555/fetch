use clap::ValueEnum;
use fetchlib::client::Client;

#[derive(Clone, ValueEnum, Debug)]
pub enum DownloadMode {
    #[clap(name = "remote")]
    RemoteFile,
    #[clap(alias = "lastmod")]
    LastModifiedFile,
}

impl ToString for DownloadMode {
    fn to_string(&self) -> String {
        match self {
            Self::RemoteFile => "remote-file",
            Self::LastModifiedFile => "last-modified",
        }
        .to_string()
    }
}
