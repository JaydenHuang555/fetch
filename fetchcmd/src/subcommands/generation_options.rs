use clap::Args;
use fetchlib::{inputs::Inputs, key::credentials::Credentials};
use fetchprofile::profile::Profile;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, Clone, Args)]
pub struct GenerationOptions {
    #[clap(long, short)]
    #[clap(visible_alias = "name")]
    #[clap(help = "profile key (identifier)")]
    pub key: String,

    #[clap(long, short)]
    #[clap(default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    #[clap(help = "remote server's address to connect to")]
    pub addr: IpAddr,

    #[clap(long, short)]
    #[clap(default_value_t = 22)]
    #[clap(help = "port to connect to on the remote server")]
    pub port: u16,

    #[clap(long, short)]
    #[clap(help = "username to connect to on the remote server")]
    pub username: String,
}

impl From<GenerationOptions> for Profile {
    fn from(options: GenerationOptions) -> Self {
        Self {
            key: options.key,
            inputs: Inputs {
                addr: SocketAddr::new(options.addr, options.port),
                credentials: Credentials {
                    username: options.username,
                    password: None,
                },
            },
        }
    }
}
