pub mod generation_options;
pub mod profile_options;
pub mod ssh_options;

pub mod error;

use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;

use clap::Subcommand;
use fetchlib::client::Client;
use fetchlib::{inputs::Inputs, key::credentials::Credentials};
use fetchprofile::manager::ProfileManager;
use fetchprofile::profile::Profile;
use rpassword::read_password;

use crate::constants::INSTANCE;
use crate::proj_dir::PROJECT_INSTANCE;

use crate::subcommands::error::SubcommandsError;
use crate::subcommands::generation_options::GenerationOptions;
use crate::subcommands::profile_options::ProfileOptions;
use crate::subcommands::ssh_options::SecureShellOptions;

#[derive(Subcommand, Debug, Clone)]
pub enum Subcommands {
    #[clap(name = "secure-shell")]
    #[clap(visible_alias = "ssh")]
    #[clap(about = "connection to a remote server over SSH")]
    SecureShell(SecureShellOptions),
    #[clap(
        name = "profile",
        about = "Connect to server over SSH with a predefined connection profile"
    )]
    Profile(ProfileOptions),
    #[clap(name = "gen", visible_alias = "generation")]
    #[clap(about = "generating profiles")]
    Generation(GenerationOptions),
}

impl Subcommands {
    pub fn get_ssh_inputs(
        &self,
        profile_manager: ProfileManager,
        pass: Option<String>,
    ) -> Result<Inputs, SubcommandsError> {
        match self {
            Self::Generation(_) => Err(SubcommandsError::AttemptToSSHInNonSSHMode),
            Self::Profile(options) => {
                if let Some(profile) = profile_manager.get_profile(options.profile_name.clone()) {
                    let mut inputs = Inputs::from(profile);
                    inputs.credentials.password = pass;
                    return Ok(inputs);
                }
                Err(SubcommandsError::InvalidProfile)
            }
            Self::SecureShell(options) => {
                let mut inputs = Inputs::from(options.clone());
                inputs.credentials.password = pass;
                Ok(inputs)
            }
        }
    }
}
