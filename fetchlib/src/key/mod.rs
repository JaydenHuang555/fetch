use rand::RngExt;
use rand::distr::Alphanumeric;
use ssh_key::PrivateKey;
use ssh_key::rand_core::OsRng;
use std::path::PathBuf;

pub mod credentials;

pub struct Secrets {
    pub pass: Option<String>,
    key: PrivateKey,
    pub fname: String,
}

impl Secrets {
    pub fn gen_key(password: Option<String>) -> PrivateKey {
        let generated = PrivateKey::random(&mut OsRng, ssh_key::Algorithm::Ed25519).unwrap();
        let pass = if password.is_none() {
            String::new()
        } else {
            password.unwrap()
        };
        generated.encrypt(&mut OsRng, pass).unwrap()
    }

    pub fn gen_name() -> String {
        let length = 20;
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    pub fn generate(pass: Option<String>) -> Self {
        let encyrpted_key = Secrets::gen_key(pass.clone());
        let fname = Secrets::gen_name();
        Self {
            pass: pass,
            key: encyrpted_key,
            fname: fname,
        }
    }

    pub fn write_openssh_to_file(&self, priv_path: PathBuf) {
        let path = priv_path.join(self.fname.clone());
        self.key
            .write_openssh_file(path.as_path(), ssh_key::LineEnding::LF)
            .unwrap();
    }

    pub fn get_pass(pass: String) -> Option<String> {
        if pass.is_empty() { None } else { Some(pass) }
    }
}
