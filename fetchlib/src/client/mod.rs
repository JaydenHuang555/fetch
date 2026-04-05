pub mod error;

use serde::Deserialize;
use serde::Serialize;
use ssh2::Session;

use crate::client::error::BlockedType;
use crate::inputs::Inputs;
use crate::remote_file_system::error::ExitCode;
use crate::sftp::Sftp;
use crate::util;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

use std::net::SocketAddr;
use std::net::TcpStream;

pub use crate::client::error::Error;

use crate::util::ssh2::remote_secure_shell_channel_close;

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
    pub fn spawn_ssh(inputs: &Inputs) -> Result<Client, Error> {
        let stream_open = TcpStream::connect(inputs.addr);
        if let Err(e) = stream_open {
            return Err(Error::connection(e, Some("Unable to open tcp stream")));
        }
        let stream = stream_open.unwrap();

        let session_open = Session::new();

        if let Err(e) = session_open {
            return Err(Error::remote_ssh2(
                e,
                Some("Unable to establish new session"),
            ));
        }

        let mut session = session_open.unwrap();
        session.set_tcp_stream(stream);

        if let Err(e) = session.handshake() {
            return Err(Error::remote_ssh2(e, Some("Failed to handshake session")));
        }

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
                return Err(Error::remote_io(
                    e,
                    ExitCode::SCP(code),
                    Some("failed to read from remote"),
                ));
            }
        }
    }

    pub fn read_file_to_file(
        &self,
        source: &Path,
        destination: &Path,
        allow_override: bool,
    ) -> Result<usize, Error> {
        let recv_operation = self.session.scp_recv(source);
        if let Err(e) = recv_operation {
            return Err(Error::remote_ssh2(e, Some("Failed to recv file")));
        }

        let (mut remote_file_channel, _) = recv_operation.unwrap();

        let mut chunk = [0u8; 512];

        if destination.exists() && !allow_override {
            return Err(Error::blocked(
                BlockedType::FilePresent,
                true,
                Some("Destination file is already present"),
            ));
        }

        let file_creation_operation = fs::File::create(destination);

        if let Err(e) = file_creation_operation {
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
                    let error_code = util::io::error_code!(e.raw_os_error());
                    return Err(Error::remote_io(
                        e,
                        ExitCode::SCP(error_code),
                        Some("Failed to read from remote source"),
                    ));
                }
            }
        }

        if let Some(e) = remote_secure_shell_channel_close!(remote_file_channel) {
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
                    e,
                    ExitCode::Session(ec),
                    Some("Failed to read remote's contents to string"),
                ))
            }
        }
    }

    pub fn sftp(&self) -> Result<Sftp, Error> {
        match self.session.sftp() {
            Ok(sfto) => Ok(Sftp::from(sfto)),
            Err(e) => Err(Error::remote_ssh2(e, Some("Failed to create sftp"))),
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
