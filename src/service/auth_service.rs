use std::collections::HashMap;

use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use async_trait::async_trait;

use crate::domain::{auth_api::{AuthToken, AuthenticationApi}, user::User};

pub struct AuthenticationService {
    user_pass_map: HashMap<i32, String>,
}

impl AuthenticationService {
    pub fn new() -> Self {
        let mut user_pass_map = HashMap::new();
        
        let salt = SaltString::generate(&mut OsRng);

        match Argon2::default().hash_password("test123".as_bytes(), &salt) {
            Ok(hashed) => {
                user_pass_map.insert(1, hashed.to_string());                
            },
            Err(_) => eprintln!("Cannot create Users passwort (init error)"),
        };
    
        AuthenticationService {
            user_pass_map,
        }
    }
}

#[async_trait]
impl AuthenticationApi for AuthenticationService {
    async fn is_password_correct(&self, user: &User, password: &str) -> bool {
        match self.user_pass_map.get(&user.id) {
            Some(hashed_user_pass) => {
                let argon2 = Argon2::default();
                let pass_hash = PasswordHash::new(&hashed_user_pass).expect("Could not create PasswordHash from &str");
                match argon2.verify_password(password.as_bytes(), &pass_hash) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            },
            None => false,
        }
    }

    fn is_authenticated(&self, auth: &dyn AuthToken) -> bool {
        auth.is_authenticated()
    }

    fn get_authenticated_user(&self, auth: &dyn AuthToken) -> Result<User, ()> {
        auth.get_authenticated_user()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{auth_api::AuthenticationApi, user::User};

    use super::AuthenticationService;


    #[tokio::test]
    async fn should_return_true_when_password_correct() {
        let auth = AuthenticationService::new();

        let user = User::new(1, "test@example.org", "Hans");

        assert!(auth.is_password_correct(&user, "test123").await, "The password should match");
    }

    #[tokio::test]
    async fn should_return_false_when_password_incorrect() {
        let auth = AuthenticationService::new();

        let user = User::new(1, "test@example.org", "Hans");

        assert!(!auth.is_password_correct(&user, "some123").await, "Password is not correct. This should return false");
    }

}