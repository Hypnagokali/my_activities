use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
#[error("Cannot query user: {msg}")]
pub struct QueryUserError {
    msg: String,
}

#[derive(Error, Debug)]
#[error("Cannot save user: {msg}")]
pub struct UserUpdateError {
    msg: String,
}

impl UserUpdateError {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.to_owned() }
    }
}

impl From<rusqlite::Error> for UserUpdateError {
    fn from(e: rusqlite::Error) -> Self {
        Self {
            msg:  e.to_string()
        }
    }
}

impl From<JoinError> for UserUpdateError {
    fn from(e: JoinError) -> Self {
        Self {
            msg:  e.to_string()
        }
    }
}

impl From<rusqlite::Error> for QueryUserError {
    fn from(e: rusqlite::Error) -> Self {
        Self {
            msg:  e.to_string()
        }
    }
}

impl From<JoinError> for QueryUserError {
    fn from(e: JoinError) -> Self {
        Self {
            msg:  e.to_string()
        }
    }
}

