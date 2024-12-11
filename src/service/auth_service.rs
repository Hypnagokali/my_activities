use std::{collections::HashMap};

use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use crate::domain::{auth_api::AuthenticationApi, user::User};

pub struct TestAuthenticationService {
    user_pass_map: HashMap<i32, String>,
}

impl TestAuthenticationService {
    pub fn new() -> Self {
        let mut user_pass_map = HashMap::new();
        
        let salt = SaltString::generate(&mut OsRng);

        match Argon2::default().hash_password("test123".as_bytes(), &salt) {
            Ok(hashed) => {
                user_pass_map.insert(1, hashed.to_string());                
            },
            Err(_) => eprintln!("Cannot create Users passwort (init error)"),
        };
    
        TestAuthenticationService {
            user_pass_map,
        }
    }
}

impl AuthenticationApi for TestAuthenticationService {
    fn is_password_correct(&self, user: &User, password: &str) -> bool {
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
}

#[cfg(test)]
mod tests {
    use crate::domain::{auth_api::AuthenticationApi, user::User};

    use super::TestAuthenticationService;


    #[test]
    fn should_return_true_when_password_correct() {
        let auth = TestAuthenticationService::new();

        let user = User::new(1, "test@example.org", "Hans");

        assert!(auth.is_password_correct(&user, "test123"), "The password should match");
    }

    #[test]
    fn should_return_false_when_password_incorrect() {
        let auth = TestAuthenticationService::new();

        let user = User::new(1, "test@example.org", "Hans");

        assert!(!auth.is_password_correct(&user, "some123"), "Password is not correct. This should return false");
    }

}