pub mod download_mode;

use crate::args::download_mode::DownloadMode;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;
use clap::ValueEnum;
use clap::ValueHint;
use fetchlib::fs::sort::FileSortType;

use crate::subcommands::Subcommands;

#[derive(Clone, ValueEnum, Debug)]
pub enum SecondGenerationOptions {
    List,
    Download,
}

impl ToString for SecondGenerationOptions {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Download => "download",
            Self::List => "list",
        })
    }
}

impl FromStr for SecondGenerationOptions {
    type Err = String;
    fn from_str(buff: &str) -> Result<Self, Self::Err> {
        match buff.to_lowercase().as_str() {
            "download" => Ok(Self::Download),
            "list" => Ok(Self::List),
            _ => Err(buff.to_string()),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, about, version = env!("VERSION"))]
pub struct FetchArgs {
    #[clap(subcommand)]
    pub action: Subcommands,

    #[clap(name = "post")]
    #[clap(long, short)]
    #[clap(value_enum)]
    pub second_gen_opts: Option<SecondGenerationOptions>,

    #[clap(long, short)]
    #[clap(default_value_t = false)]
    pub size: bool,

    #[clap(long, short)]
    #[clap(required_if_eq("post", "list"))]
    #[clap(required_if_eq("post", "download"))]
    #[clap(required_if_eq("size", "true"))]
    pub remote_path: Option<PathBuf>,

    #[clap(long)]
    #[clap(required_if_eq("post", "download"))]
    #[clap(value_hint = ValueHint::FilePath)]
    pub local_path: Option<PathBuf>,

    #[clap(long)]
    #[clap(default_value_t = FileSortType::LastModified)]
    pub sort_mode: FileSortType,

    #[clap(long)]
    #[clap(default_value_t = DownloadMode::RemoteFile)]
    pub download_mode: DownloadMode,
}

impl FetchArgs {}
