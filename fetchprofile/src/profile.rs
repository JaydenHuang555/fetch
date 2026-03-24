use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use std::path::Path;
use std::{
    error::Error,
    fmt::{self, Display},
    io,
};

use serde::{Deserialize, Serialize};

use fetchlib::inputs::Inputs;

use crate::error::ProfileError;

#[derive(Serialize, Deserialize, Clone)]
pub struct Profile {
    pub key: String,
    pub inputs: Inputs,
}

impl Profile {
    pub fn serialize_json(&self) -> Result<String, ProfileError> {
        match serde_json::to_string(self) {
            Ok(contents) => Ok(contents),
            Err(e) => Err(ProfileError::SerializeErr(e)),
        }
    }

    pub fn deserialize_from_json(path: &Path) -> Result<Profile, ProfileError> {
        match std::fs::read_to_string(path) {
            Ok(contents) => match serde_json::from_str(contents.as_str()) {
                Ok(profile) => Ok(profile),
                Err(e) => Err(ProfileError::DeserializeErr(e)),
            },
            Err(e) => Err(ProfileError::FileIO(e)),
        }
    }
}

impl From<Profile> for Inputs {
    fn from(profile: Profile) -> Self {
        profile.inputs
    }
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
