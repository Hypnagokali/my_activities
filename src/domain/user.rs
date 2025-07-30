use authfix::session::AccountInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
}

impl AccountInfo for User {}

impl User {
    pub fn new(id: i32, email: String, name: String) -> Self {
        User {
            id,
            email: email,
            name: name,
        }
    }
    
}

pub struct Credentials {
    pub id: i32,
    pub password: String,
    pub mfa_config: Option<Mfa>,
    pub user_id: i32,

}

impl Credentials {
    pub fn new(id: i32, password: String, user_id: i32) -> Self {
        Self {
            id,
            password,
            mfa_config: None,
            user_id
        }
    }

    pub fn set_mfa(&mut self, mfa_config: Mfa) {
        self.mfa_config = Some(mfa_config);
    }
}

pub struct Mfa {
    pub mfa_id: String,
    pub secret: Option<String>
}

impl Mfa {
    pub fn new(mfa_id: &str) -> Self {
        Self {
            mfa_id: mfa_id.to_owned(),
            secret: None,
        }
    }
    pub fn with_secret(mfa_id: &str, secret: &str) -> Self {
        Self {
            mfa_id: mfa_id.to_owned(),
            secret: Some(secret.to_owned()),
        }
    }    
}