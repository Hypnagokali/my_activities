use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
}

impl User {
    pub fn test_user() -> Self {
        User {
            id: 1,
            email: "test@example.org".to_owned(),
            name: "Hans".to_owned(),
        }
    }
    pub fn new(id: i32, email: &str, name: &str) -> Self {
        User {
            id,
            email: email.to_owned(),
            name: name.to_owned(),
        }
    }
}
