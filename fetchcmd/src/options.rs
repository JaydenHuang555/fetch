use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

use clap::Args;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use fetchlib::client::Client;
use fetchlib::{inputs::Inputs, key::credentials::Credentials};
use fetchprofile::manager::ProfileManager;
use fetchprofile::profile::Profile;
use rpassword::read_password;

use crate::proj_dir::PROJECT_INSTANCE;

#[derive(Subcommand, Debug, Clone)]
pub enum Action {
    #[clap(name = "secure-shell")]
    #[clap(visible_alias = "ssh")]
    #[clap(about = "connection to a remote server over SSH")]
    SecureShell {
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
    },
    #[clap(
        name = "profile",
        about = "Connect to server over SSH with a predefined connection profile"
    )]
    Profile {
        #[clap(long, short)]
        #[clap(help = "profile name")]
        profile_name: String,
    },
    #[clap(name = "gen", visible_alias = "generation")]
    #[clap(about = "generating profiles")]
    Generation {
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
    },
}

impl Action {
    pub fn get_ssh_input(&self, pass: Option<String>) -> Option<Inputs> {
        match self {
            Self::SecureShell {
                addr,
                port,
                username,
            } => Some(Inputs {
                addr: SocketAddr::new(*addr, *port),
                credentials: Credentials {
                    username: username.clone(),
                    password: pass,
                },
            }),
            Self::Profile { profile_name } => {
                let profiles_storage = PROJECT_INSTANCE
                    .lock()
                    .unwrap()
                    .dir
                    .cache_dir()
                    .join("profiles");
                let mut p = profiles_storage.join(profile_name);
                p.set_extension("json");
                let path = p.as_path();
                let deserialized = Profile::deserialize_from_json(path);
                if deserialized.is_err() {
                    return None;
                }
                let mut profile = deserialized.unwrap();
                profile.inputs.credentials.password = pass;
                return Some(profile.inputs);
            }
            _ => None,
        }
    }

    pub fn get_updated_manager(&self) -> ProfileManager {
        let path_buff = PROJECT_INSTANCE
            .lock()
            .unwrap()
            .dir
            .cache_dir()
            .join("profiles");
        let path = path_buff.as_path();
        ProfileManager::load(path).unwrap()
    }

    pub fn get_pass(&self) -> Option<String> {
        print!("Please enter password: ");
        std::io::stdout().flush().unwrap();
        let read = read_password().unwrap();
        if read.is_empty() { None } else { Some(read) }
    }

    pub fn connect_ssh(&self, inputs: &Inputs) {
        let mut client = Client::spawn(inputs).unwrap();
        let output = client.run_cmd("cd ~; ls -a");
        println!("{:?}", output);
    }

    pub fn execute(&self) {
        match self {
            Self::Generation {
                key,
                addr,
                port,
                username,
            } => {
                let manager = self.get_updated_manager();
                manager
                    .add_profile(
                        &Profile {
                            key: key.clone(),
                            inputs: Inputs {
                                addr: SocketAddr::new(*addr, *port),
                                credentials: Credentials {
                                    username: username.clone(),
                                    password: None,
                                },
                            },
                        },
                        true,
                    )
                    .unwrap();
            }
            Self::Profile { profile_name } => {
                let pass = self.get_pass();
                let inputs = self.get_ssh_input(pass).unwrap();
                self.connect_ssh(&inputs);
            }
            Self::SecureShell {
                addr,
                port,
                username,
            } => {
                let pass = self.get_pass();
                let inputs = self.get_ssh_input(pass);
                self.connect_ssh(&inputs.unwrap());
            }
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, about, version = env!("VERSION"))]
pub struct Options {
    #[clap(subcommand)]
    pub action: Action,
}

impl Options {
    pub fn convert_to_inputs(&self, pass: Option<String>) -> Option<Inputs> {
        self.action.get_ssh_input(pass)
    }
}
