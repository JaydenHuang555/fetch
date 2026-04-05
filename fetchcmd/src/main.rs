pub mod args;
pub mod proj_dir;

use std::{
    io::{Write, stdout},
    path::PathBuf,
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
    args::{FetchArgs, SecondGenerationOptions, download_mode::DownloadMode},
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

pub fn download(
    client: Client,
    download_mode: DownloadMode,
    local_path: Option<PathBuf>,
    remote_path: Option<PathBuf>,
) -> Option<ExitCode> {
    match download_mode {
        DownloadMode::RemoteFile => {
            let local = local_path.unwrap();
            let remote = remote_path.unwrap();
            println!("Downloading {} to {}", remote.display(), local.display());
            if let Err(e) = client.read_file_to_file(remote.as_path(), local.as_path(), false) {
                eprintln!("Failure detected when downloading file: {}", e);
                return Some(ExitCode::from(10));
            }
            println!("Finished downloading file");
        }
        DownloadMode::LastModifiedFile => {
            let remote = remote_path.unwrap();
            let local = local_path.unwrap();
            let sftp_operation = client.sftp();
            if let Err(e) = sftp_operation {
                eprintln!("Failed to get sftp due to {}", e);
                return Some(ExitCode::from(75));
            }
            println!("Finished starting sftp");
            let sftp = sftp_operation.unwrap();
            if !sftp.isdir(remote.as_path()) {
                eprintln!("Given remote path is not a directory");
                return Some(ExitCode::from(8));
            }
            let list_op = sftp.listdir(remote.as_path());
            if let Err(e) = list_op {
                eprintln!("Unable to fetch directory files due to {}", e);
                return Some(ExitCode::from(5));
            }
            let latest = sftp.last_mod_file(remote.as_path()).unwrap();
            println!("Found latest to be {:?}", latest);
            println!(
                "Downloading {} to {}",
                latest.path.display(),
                local.display()
            );
            if let Err(e) = client.read_file_to_file(latest.path.as_path(), local.as_path(), true) {
                eprintln!("Failure detected when downloading file: {}", e);
                return Some(ExitCode::from(10));
            }
            println!("Finished downloading file");
        }
    }
    None
}

fn handle_ssh_second_generation(client: Client, args: FetchArgs) -> Option<ExitCode> {
    if args.size {
        println!("Getting size");
        let sftp_operation = client.sftp();
        if let Err(e) = sftp_operation {
            eprintln!("Failed to get sftp due to {}", e);
            return Some(ExitCode::from(75));
        }
        println!("Finished starting sftp");
        let sftp = sftp_operation.unwrap();
        println!(
            "{:?}",
            sftp.dirsize(args.remote_path.clone().unwrap().as_path())
        );
    }
    if let Some(second_gen_opts) = args.second_gen_opts {
        match second_gen_opts {
            SecondGenerationOptions::List => {
                println!("Fetching Files");
                println!("Starting sftp");
                let sftp_operation = client.sftp();
                if let Err(e) = sftp_operation {
                    eprintln!("Failed to get sftp due to {}", e);
                    return Some(ExitCode::from(75));
                }
                let sftp = sftp_operation.unwrap();
                println!("Finished starting sftp");
                let list_op = sftp.listdir(args.remote_path.clone().unwrap().as_path());
                if let Err(e) = list_op {
                    eprintln!("Unable to list directory due to {}", e);
                    return Some(ExitCode::from(7));
                }
                let mut meta_data_list = list_op.unwrap();
                args.sort_mode.sort_vector(&mut meta_data_list);
                for e in meta_data_list {
                    println!("{:?}", e);
                }
            }
            SecondGenerationOptions::Download => {
                let result = download(
                    client,
                    args.download_mode,
                    args.local_path,
                    args.remote_path,
                );
                if result.is_some() {
                    return result;
                }
            }
        }
    }
    None
}

fn secure_shell(profile_manager: ProfileManager, args: FetchArgs) -> Option<ExitCode> {
    print!("Please enter the password: ");
    stdout().flush().unwrap();
    let pass = Secrets::get_pass(read_password().unwrap());
    let inputs = args.action.get_ssh_inputs(profile_manager, pass).unwrap();

    match Client::spawn_ssh(&inputs) {
        Ok(client) => handle_ssh_second_generation(client, args),
        Err(e) => {
            eprintln!("Unable to open client to {} due to {}", inputs.uri(), e);
            return Some(ExitCode::from(18));
        }
    }
}

fn fetchcmd(args: FetchArgs) -> ExitCode {
    let profile_manager = get_profile_manager();
    match args.action {
        Subcommands::Generation(options) => generation(options),
        Subcommands::SecureShell(_) | Subcommands::Profile(_) => {
            if let Some(e) = secure_shell(profile_manager, args) {
                return e;
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
