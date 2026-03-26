pub mod args;
pub mod proj_dir;

use std::{
    io::{Write, stdout},
    process::ExitCode,
};

use directories::ProjectDirs;
use fetchlib::{key::Secrets, remote_file_system::RemoteFileSystem};
use fetchprofile::manager::ProfileManager;
use rpassword::read_password;

use clap::Parser;
use fetchlib::client::Client;
use serde::de;

use crate::{
    args::{FetchArgs, SecondGenerationOptions},
    constants::INSTANCE,
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

fn handle_ssh_second_generation(client: Client, args: FetchArgs) -> Option<ExitCode> {
    if args.size {
        println!("Getting size");
        println!("{:?}", client.dirsize(args.remote_path.clone().unwrap()));
    }
    if let Some(second_gen_opts) = args.second_gen_opts {
        match second_gen_opts {
            SecondGenerationOptions::List => {
                println!("Fetching Files");
                let mut meta_data_list = client.listdir(args.remote_path.unwrap());
                args.sort_mode.sort(&mut meta_data_list);
                for e in meta_data_list {
                    println!("{:?}", e);
                }
            }
            SecondGenerationOptions::Download => {
                let remote_path_buff = args.remote_path.unwrap().clone();
                let local_path_buff = args.local_path.unwrap().clone();
                let local_path = local_path_buff.as_path();
                let remote_path = remote_path_buff.as_path();
                if !client.path_exists(remote_path.to_path_buf()) {
                    eprintln!("Given path does not exist");
                    return Some(ExitCode::from(3)); // TODO: add constants for exit codes
                }
                println!("Download file");
                client.read_file_to_file(remote_path, local_path);
            }
        }
    }
    None
}

fn fetchcmd(args: FetchArgs) -> ExitCode {
    let profile_manager = get_profile_manager();
    match args.action {
        Subcommands::Generation(options) => generation(options),
        Subcommands::SecureShell(_) | Subcommands::Profile(_) => {
            print!("Please enter the password: ");
            stdout().flush().unwrap();
            let pass = Secrets::get_pass(read_password().unwrap());
            let inputs = args.action.get_ssh_inputs(profile_manager, pass).unwrap();
            let client = Client::spawn(&inputs).unwrap();
            if let Some(exit_code) = handle_ssh_second_generation(client, args) {
                return exit_code;
            }
        }
    }
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let proj_directory = ProjectDirs::from("com", "Jayden", "fetch").unwrap();
    let args = FetchArgs::parse();
    fetchcmd(args)
}
