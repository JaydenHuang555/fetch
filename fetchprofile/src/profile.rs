use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use std::{
    error::Error,
    fmt::{self, Display},
    io,
};

use serde::{Deserialize, Serialize};

use fetchlib::inputs::Inputs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Profile {
    pub key: String,
    pub inputs: Inputs,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            key: String::new(),
            inputs: Inputs::default(),
        }
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{key: {}}}", self.key);
        writeln!(f, "{{username: {}}}", self.inputs.credentials.username);
        writeln!(f, "{{addr: {}}}", self.inputs.addr)
    }
}
