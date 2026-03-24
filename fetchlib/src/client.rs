use rand::rng;
use serde::Deserialize;
use serde::Serialize;
use ssh_key::PrivateKey;
use ssh2::Session;

use std::io::prelude::*;
use std::path::Path;

use crate::inputs::Inputs;

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

        //        session.userauth_agent(inputs.username.as_str()).unwrap();
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
