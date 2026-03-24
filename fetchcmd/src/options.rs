use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use clap::Parser;
use fetchlib::{inputs::Inputs, key::credentials::Credentials};

#[derive(Parser, Debug)]
pub struct Options {
    #[clap(long, short, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    pub addr: IpAddr,

    #[clap(long, short, default_value_t = 22)]
    pub port: u16,

    #[clap(long, short)]
    pub username: String,
}

impl Options {
    pub fn convert_to_inputs(&self, pass: Option<String>) -> Inputs {
        Inputs {
            addr: SocketAddr::new(self.addr, self.port),
            credentials: Credentials {
                username: self.username.clone(),
                password: pass,
            },
        }
    }
}
