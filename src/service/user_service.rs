use std::collections::HashMap;

use crate::{domain::{user::User, user_api::UserApi}, error::errors::NotFoundError};

pub struct UserService {
    users: HashMap<String, User>,
}

impl UserService {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("test@example.org".to_owned(), User::new(123, "test@example.org", "Hans"));

        return UserService {
            users,
        }
    }
}

impl UserApi for UserService {
    fn find_by_email(&self, email: &str) -> Result<User, NotFoundError> {
        match self.users.get(email.to_lowercase().trim()) {
            Some(user) => Ok(user.clone()),
            None => Err(NotFoundError::new("User not found")),
        }


    }
}