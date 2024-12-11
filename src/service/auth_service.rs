use std::{collections::HashMap};

use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHasher};

use crate::domain::{auth_api::AuthenticationApi, user::User};

pub struct TestAuthenticationService {
    user_pass_map: HashMap<i32, String>,
    user_salt_map: HashMap<i32, String>,
}

impl TestAuthenticationService {
    pub fn new() -> Self {
        let mut user_pass_map = HashMap::new();
        let mut user_salt_map = HashMap::new();
        
        let salt = SaltString::generate(&mut OsRng);
        println!("Salt for Testuser: {}", salt);
        user_salt_map.insert(1,salt.to_string());

        match Argon2::default().hash_password("test123".as_bytes(), &salt) {
            Ok(hashed) => {
                user_pass_map.insert(1, hashed.to_string());                
            },
            Err(_) => eprintln!("Cannot create Users passwort (init error)"),
        };
    
        TestAuthenticationService {
            user_pass_map,
            user_salt_map,
        }
    }

    fn hash_string(&self, input: &str, salt: &SaltString) -> String {
        let argon2 = Argon2::default();
        let hashed = argon2.hash_password(input.as_bytes(), salt);

        hashed.expect("Could not genrate hash").to_string()
    }

    fn check_password_with_salt(&self, user_id: &i32, password: &str, salt: &SaltString) -> bool {
        match self.user_pass_map.get(user_id) {
            Some(hashed_user_pass) => {
                let check_against = self.hash_string(password, salt);

                *hashed_user_pass == check_against
            },
            None => false,
        }
    }
}

impl AuthenticationApi for TestAuthenticationService {
    fn is_password_correct(&self, user: &User, password: &str) -> bool {
        match self.user_salt_map.get(&user.id) {
            Some(salt) => {
                match SaltString::from_b64(&salt) {
                    Ok(salt_string) => self.check_password_with_salt(&user.id, password, &salt_string),
                    Err(_) => {
                        eprint!("Could not make SaltString from str");
                        return false;
                    },
                }
            } 
            None => false
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