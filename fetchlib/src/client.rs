use rand::rng;
use serde::Deserialize;
use serde::Serialize;
use ssh2::Session;
use ssh_key::PrivateKey;

use std::fs;
use std::io;
use std::io::prelude::*;
use std::path;
use std::path::Path;
use std::path::PathBuf;

use crate::inputs::Inputs;
use crate::metadata::FileMetaData;
use crate::remote_file_system::RemoteFileSystem;

use std::net::SocketAddr;
use std::net::TcpStream;

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
    pub fn spawn(inputs: &Inputs) -> Result<Client, bool> {
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
            return Result::Err(false);
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

    pub fn read_file_to_vec(&self, path: &Path, destination: &mut Vec<u8>) {
        let (mut remote_file_channel, stat) = self.session.scp_recv(path).unwrap();
        remote_file_channel.read_to_end(destination).unwrap();
        remote_file_channel.send_eof().unwrap();
        remote_file_channel.wait_eof().unwrap();
        remote_file_channel.close().unwrap();
        remote_file_channel.wait_close();
    }

    pub fn read_file_to_file(&self, source: &Path, destination: &Path) {
        let (mut remote_file_channel, stat) = self.session.scp_recv(source).unwrap();

        let mut chunk = [0u8; 512];

        if destination.exists() {
            fs::remove_file(destination).unwrap();
        }

        let mut fd = fs::File::create_new(destination).unwrap();

        loop {
            let read = remote_file_channel.read(&mut chunk).unwrap();
            if read == 0 {
                break;
            }
            fd.write_all(&chunk).unwrap();
        }

        remote_file_channel.send_eof().unwrap();
        remote_file_channel.wait_eof().unwrap();
        remote_file_channel.close().unwrap();
        remote_file_channel.wait_close().unwrap();
    }

    pub fn run_cmd<S: AsRef<str>>(&mut self, cmd: S) -> (i32, String) {
        let mut channel = self.session.channel_session().unwrap();
        channel.exec(cmd.as_ref()).unwrap();
        let mut output = String::new();
        channel.read_to_string(&mut output).unwrap();
        channel.wait_close().unwrap();
        (channel.exit_status().unwrap(), output)
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

impl RemoteFileSystem for Client {
    fn file_metadata(&self, fpath: PathBuf) -> FileMetaData {
        let stfp = self.session.sftp().unwrap();
        let stat = stfp.stat(fpath.as_path()).unwrap();
        let mut meta_data = FileMetaData::from(stat);
        meta_data.path = fpath;
        meta_data
    }

    fn listdir(&self, path: PathBuf) -> Vec<FileMetaData> {
        let sftp = self.session.sftp().unwrap();
        let contents = sftp.readdir(path).unwrap();
        let output: Vec<FileMetaData> = contents
            .into_iter()
            .map(|c| {
                let mut m = FileMetaData::from(c.1);
                m.path = c.0;
                m
            })
            .collect();
        output
    }

    fn path_exists(&self, path: PathBuf) -> bool {
        let sftp = self.session.sftp().unwrap();
        sftp.stat(path.as_path()).is_ok()
    }
}
