pub mod helpers;
mod remote;

use serde::Deserialize;
use serde::Serialize;
use ssh2::Session;

use crate::inputs::Inputs;
use crate::remote_file_system::error::ExitCode;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

use std::net::SocketAddr;
use std::net::TcpStream;

use crate::error::Error;

use crate::client::helpers::remote_secure_shell_channel_close;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientInfo {
    pub username: String,
    pub addr: SocketAddr,
}

pub struct Client {
    pub info: ClientInfo,
    pub session: Session,
}

impl Client {
    pub fn spawn(inputs: &Inputs) -> Result<Client, Error> {
        let stream = TcpStream::connect(inputs.addr).unwrap();
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(stream);
        session.handshake().unwrap();

        session
            .userauth_password(
                inputs.credentials.username.as_str(),
                inputs.credentials.pass_as_str().as_str(),
            )
            .unwrap();

        if !session.authenticated() {
            return Result::Err(Error::unathenticated(Some("Failed to authenticate client")));
        }

        // TODO: handle agent

        return Result::Ok(Client {
            info: ClientInfo {
                username: inputs.credentials.username.clone(),
                addr: inputs.addr,
            },
            session: session,
        });
    }

    pub fn read_file_to_vec(&self, path: &Path, destination: &mut Vec<u8>) -> Result<usize, Error> {
        let recv_operation = self.session.scp_recv(path);
        if let Err(e) = recv_operation {
            return Err(Error::remote_ssh2(
                e,
                Some("Failed to open channel to remote file"),
            ));
        }
        let (mut remote_file_channel, _) = recv_operation.unwrap();
        let read_operation = remote_file_channel.read_to_end(destination);
        match read_operation {
            Ok(read_bytes) => {
                if let Some(e) = remote_secure_shell_channel_close!(remote_file_channel) {
                    return Err(Error::remote_ssh2(e, Some("Failed to close remote server")));
                }
                return Ok(read_bytes);
            }
            Err(e) => {
                let code = { if let Some(c) = e.raw_os_error() { c } else { 1 } };
                return Err(Error::remote_io(ExitCode::SCP(code), None));
            }
        }
    }

    pub fn read_file_to_file(&self, source: &Path, destination: &Path) -> Result<usize, Error> {
        let recv_operation = self.session.scp_recv(source);
        if let Err(e) = recv_operation {
            return Err(Error::remote_ssh2(e, None));
        }

        let (mut remote_file_channel, _) = recv_operation.unwrap();

        let mut chunk = [0u8; 512];

        if destination.exists() {
            if let Err(e) = fs::remove_file(destination) {
                return Err(Error::local_fs(
                    e,
                    Some("Unable to remove the destination file"),
                ));
            }
        }

        let file_creation_operation = fs::File::create(destination);

        if let Err(e) = fs::File::create_new(destination) {
            return Err(Error::local_fs(
                e,
                Some("Unable to create the destination file"),
            ));
        }

        let mut fd = file_creation_operation.unwrap();

        let mut transfered_bytes_total = 0;

        loop {
            match remote_file_channel.read(&mut chunk) {
                Ok(read_bytes) => {
                    if read_bytes == 0 {
                        break;
                    }
                    if let Err(e) = fd.write_all(&chunk) {
                        return Err(Error::local_fs(e, Some("Failed to write to destination")));
                    }
                    transfered_bytes_total = transfered_bytes_total + read_bytes;
                }
                Err(e) => {
                    let error_code = {
                        {
                            if let Some(code) = e.raw_os_error() {
                                code
                            } else {
                                1
                            }
                        }
                    };
                    return Err(Error::remote_io(
                        ExitCode::SCP(error_code),
                        Some("Failed to read from remote source"),
                    ));
                }
            }
        }

        if let Some(e) = helpers::remote_secure_shell_channel_close!(remote_file_channel) {
            return Err(Error::remote_ssh2(
                e,
                Some("Failed to close remote file channel"),
            ));
        }
        Ok(transfered_bytes_total)
    }

    pub fn run_cmd<S: AsRef<str>>(&mut self, cmd: S) -> Result<(i32, String), Error> {
        let channel_op = self.session.channel_session();
        if let Err(e) = channel_op {
            return Err(Error::remote_ssh2(e, Some("Failed to open channel stream")));
        }
        let mut channel = channel_op.unwrap();

        if let Err(e) = channel.exec(cmd.as_ref()) {
            return Err(Error::remote_ssh2(e, Some("Failed to execute command")));
        }

        let mut output = String::new();

        match channel.read_to_string(&mut output) {
            Ok(_) => {
                if let Err(e) = channel.wait_close() {
                    return Err(Error::remote_ssh2(e, Some("Failed to close channel")));
                }
                let exit_stat_op = channel.exit_status();
                if let Err(e) = exit_stat_op {
                    return Err(Error::remote_ssh2(e, None));
                }
                let exit_stat = exit_stat_op.unwrap();
                Ok((exit_stat, output))
            }
            Err(e) => {
                let ec = { if let Some(c) = e.raw_os_error() { c } else { 1 } };
                Err(Error::remote_io(
                    ExitCode::Session(ec),
                    Some("Failed to read remote's contents to string"),
                ))
            }
        }
    }

    pub fn contains_username_key(username: String) -> bool {
        let mut session = Session::new().unwrap();
        let stream = TcpStream::connect("127.0.0.1:22").unwrap();
        session.set_tcp_stream(stream);
        session.handshake().unwrap();
        let agent = session.agent().unwrap();
        for identity in agent.identities().unwrap() {
            if agent.userauth(username.as_str(), &identity).is_ok() {
                return true;
            }
        }
        false
    }
}
