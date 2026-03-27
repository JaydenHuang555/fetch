use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};

use crate::key::credentials::Credentials;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Inputs {
    pub addr: SocketAddr,
    pub credentials: Credentials,
}

impl Inputs {
    pub fn uri(&self) -> String {
        format!(
            "{}@{}:{}",
            self.credentials.username,
            self.addr.ip(),
            self.addr.port()
        )
    }
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 22),
            credentials: Credentials::default(),
        }
    }
}
