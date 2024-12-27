use std::collections::HashMap;

use crate::{domain::{user::User, user_api::UserApi}, error::errors::NotFoundError};

pub struct UserService {
    users: HashMap<String, User>,
}

impl UserService {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        let test_user= User::test_user();
        users.insert(test_user.email.clone(), test_user);

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