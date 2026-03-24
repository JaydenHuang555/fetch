use clap::Args;

use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone, Args)]
pub struct GenerationOptions {
    #[clap(long, short)]
    #[clap(visible_alias = "name")]
    #[clap(help = "profile key (identifier)")]
    key: String,

    #[clap(long, short)]
    #[clap(default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    #[clap(help = "remote server's address to connect to")]
    addr: IpAddr,

    #[clap(long, short)]
    #[clap(default_value_t = 22)]
    #[clap(help = "port to connect to on the remote server")]
    port: u16,

    #[clap(long, short)]
    #[clap(help = "username to connect to on the remote server")]
    username: String,
}

