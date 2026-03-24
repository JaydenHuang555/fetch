use std::fmt::Display;
use std::fmt::Formatter;
use std::fs;
use std::io;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Args;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use clap::ValueHint;
use fetchlib::client::Client;
use fetchlib::{inputs::Inputs, key::credentials::Credentials};
use fetchprofile::manager::ProfileManager;
use fetchprofile::profile::Profile;
use rpassword::read_password;

use crate::constants::INSTANCE;
use crate::proj_dir::PROJECT_INSTANCE;

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
    #[clap(required_if_eq("post", "list"))]
    #[clap(required_if_eq("post", "download"))]
    pub remote_path: Option<PathBuf>,

    #[clap(long)]
    #[clap(required_if_eq("post", "download"))]
    #[clap(value_hint = ValueHint::FilePath)]
    pub local_path: Option<PathBuf>,
}

impl FetchArgs {}
