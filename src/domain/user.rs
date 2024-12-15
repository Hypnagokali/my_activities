use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
}

impl User {
    pub fn new(id: i32, email: &str, name: &str) -> Self {
        User {
            id,
            email: email.to_owned(),
            name: name.to_owned(),
        }
    }
}