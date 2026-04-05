use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: Option<String>,
}

impl Serialize for Credentials {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Credentials", 2)?;
        state.serialize_field("username", &self.username)?;
        let pass;
        if let Some(content) = self.password.clone() {
            pass = content;
        } else {
            pass = String::from("");
        }
        state.serialize_field("password", &pass)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Credentials {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            username: String,
            password: String,
        }
        let helper = Helper::deserialize(deserializer).unwrap();
        let password = if helper.password.is_empty() {
            None
        } else {
            Some(helper.password)
        };
        Ok(Credentials {
            username: helper.username,
            password,
        })
    }
}

impl Credentials {
    pub fn generate_opt_pass(content: String) -> Option<String> {
        if content.is_empty() {
            None
        } else {
            Some(content)
        }
    }

    pub fn from(username: String) -> Self {
        Self {
            username,
            password: None,
        }
    }

    pub fn from_with_pass(username: String, password: String) -> Self {
        Self {
            username,
            password: Self::generate_opt_pass(password),
        }
    }

    pub fn pass_as_str(&self) -> String {
        if self.password.is_some() {
            self.password.clone().unwrap()
        } else {
            String::from("")
        }
    }
}

impl Default for Credentials {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: None,
        }
    }
}
