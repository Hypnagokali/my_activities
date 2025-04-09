use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
}

impl User {
    pub fn new(id: i32, email: String, name: String) -> Self {
        User {
            id,
            email: email,
            name: name,
        }
    }
}

#[allow(dead_code)]
pub struct Credentials {
    pub id: i32,
    pub password: String,
    pub user_id: i32,

}
impl Credentials {
    pub fn new(id: i32, password: String, user_id: i32) -> Self {
        Self {
            id,
            password,
            user_id
        }
    }
}
