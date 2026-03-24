use std::net::{Ipv4Addr, SocketAddr};

pub mod options;
pub mod proj_dir;

use directories::ProjectDirs;
use fetchlib::inputs::Inputs;
use fetchlib::{client::Client, key::credentials::Credentials};
use fetchprofile::manager::ProfileManager;
use fetchprofile::profile::Profile;

fn main() {
    let proj_directory = ProjectDirs::from("com", "Jayden", "fetch").unwrap();

    let inputs = Inputs {
        addr: SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(10, 16, 78, 2)), 22),
        credentials: Credentials {
            username: String::from("lvuser"),
            password: Some(String::from("10.16.78.2")),
        },
    };

    let path_buff = proj_directory.data_dir().join("profile");
    let path = path_buff.as_path();

    let mut client = Client::spawn(inputs).unwrap();

    let output = client.run_cmd("cd ~; ls ");
    println!("Exit Code: {}", output.0);
    println!("Content: {}", output.1);
}
