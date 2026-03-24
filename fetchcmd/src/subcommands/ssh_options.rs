use clap::Args;
use fetchlib::inputs::Inputs;
use fetchlib::key::credentials::Credentials;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;

#[derive(Args, Debug, Clone)]
pub struct SecureShellOptions {
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

impl From<SecureShellOptions> for Inputs {
    fn from(options: SecureShellOptions) -> Inputs {
        Inputs {
            addr: SocketAddr::new(options.addr, options.port),
            credentials: Credentials {
                username: options.username,
                password: None,
            },
        }
    }
}
