pub mod options;
pub mod proj_dir;

use std::io::{Write, stdout};

use directories::ProjectDirs;
use fetchlib::{key::Secrets, remote_file_system::RemoteFileSystem};
use fetchprofile::manager::ProfileManager;
use rpassword::read_password;

use clap::Parser;
use fetchlib::client::Client;

use crate::{
    constants::INSTANCE,
    options::{FetchArgs, SecondGenerationOptions},
    subcommands::generation_options::GenerationOptions,
};

pub mod constants;
pub mod subcommands;

use crate::constants::constants_instance;

use subcommands::Subcommands;

fn generation(options: GenerationOptions) {}

fn get_profile_manager() -> ProfileManager {
    let profile_path_buff = constants_instance!().profiles_path.clone();
    ProfileManager::load(profile_path_buff.as_path()).unwrap()
}

fn handle_ssh_second_generation(client: Client, args: FetchArgs) {
    if let Some(second_gen_opts) = args.second_gen_opts {
        match second_gen_opts {
            SecondGenerationOptions::List => {
                let meta_data_list = client.listdir(args.remote_path.unwrap());
                for e in meta_data_list {
                    println!("{:?}", e);
                }
                println!("Finished");
            }
            _ => {}
        }
    }
}

fn fetchcmd(args: FetchArgs) {
    let profile_manager = get_profile_manager();
    match args.action {
        Subcommands::Generation(options) => generation(options),
        Subcommands::SecureShell(_) | Subcommands::Profile(_) => {
            print!("Please enter the password: ");
            stdout().flush().unwrap();
            let pass = Secrets::get_pass(read_password().unwrap());
            let inputs = args.action.get_ssh_inputs(profile_manager, pass).unwrap();
            let client = Client::spawn(&inputs).unwrap();
            handle_ssh_second_generation(client, args);
        }
    }
}

fn main() {
    let proj_directory = ProjectDirs::from("com", "Jayden", "fetch").unwrap();
    let args = FetchArgs::parse();
    fetchcmd(args);
}
