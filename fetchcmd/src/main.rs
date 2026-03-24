pub mod options;
pub mod proj_dir;

use directories::ProjectDirs;
use fetchlib::key::Secrets;
use rpassword::read_password;

use clap::Parser;
use fetchlib::client::Client;

use crate::options::Options;

fn main() {
    let proj_directory = ProjectDirs::from("com", "Jayden", "fetch").unwrap();
    let options = Options::parse();
    options.action.execute();
}
