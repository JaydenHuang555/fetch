pub mod options;
pub mod proj_dir;

// use directories::ProjectDirs;
use fetchlib::key::Secrets;
use rpassword::read_password;

use clap::Parser;
use fetchlib::client::Client;

use crate::options::Options;

fn main() {
    // let proj_directory = ProjectDirs::from("com", "Jayden", "fetch").unwrap();

    let options = Options::parse();

    let pass_read = read_password().unwrap();

    let pass = Secrets::get_pass(pass_read);

    let inputs = options.convert_to_inputs(pass);

    let mut client = Client::spawn(inputs).unwrap();

    let output = client.run_cmd("cd ~; ls ");
    println!("Exit Code: {}", output.0);
    println!("Content: {}", output.1);
}
