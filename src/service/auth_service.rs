use std::{collections::HashMap, sync::Arc};

use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use async_trait::async_trait;
use authfix::login::LoadUserService;

use crate::domain::{auth_api::AuthenticationApi, user::User, user_api::UserApi};

pub struct AuthenticationService<U: UserApi> {
    user_pass_map: HashMap<i32, String>,
    user_api: Arc<U>
}

impl<U: UserApi> AuthenticationService<U> {
    pub fn new(user_api: Arc<U>) -> Self {
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
            user_api,
        }
    }
}

#[async_trait]
impl<U: UserApi> AuthenticationApi for AuthenticationService<U> {
    async fn is_password_correct(&self, user: &User, password: &str) -> bool {
        println!("Check if password correct!");
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

impl<U: UserApi> LoadUserService for AuthenticationService<U> {
    type User = User;

    fn load_user(
        &self,
        login_token: &authfix::login::LoginToken,
    ) -> futures::future::LocalBoxFuture<'_, Result<Self::User, authfix::login::LoadUserError>> {
        let email = login_token.username.clone();
        let password = login_token.password.clone();
        Box::pin(async move {
            match self.user_api.find_by_email(&email).await {
                Ok(user) => {
                    if self.is_password_correct(&user, &password).await {
                        Ok(user)
                    } else {
                        Err(authfix::login::LoadUserError::LoginFailed)
                    }
                },
                Err(_) => Err(authfix::login::LoadUserError::LoginFailed),
            }
        })

    }

    fn on_success_handler(
        &self,
        _req: &actix_web::HttpRequest,
        user: &Self::User,
    ) -> futures::future::LocalBoxFuture<'_, Result<(), authfix::login::HandlerError>> {
        println!("Success: user -> {}", user.name);
        Box::pin(async {
            Ok(())
        })
    }

    fn on_error_handler(&self, _req: &actix_web::HttpRequest) -> futures::future::LocalBoxFuture<'_, Result<(), authfix::login::HandlerError>> {
        Box::pin(async {
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{domain::{auth_api::AuthenticationApi, user::User}, service::user_service::UserService};

    use super::AuthenticationService;


    #[tokio::test]
    async fn should_return_true_when_password_correct() {
        let auth = AuthenticationService::new(Arc::new(UserService::new()));

        let user = User::new(1, "test@example.org", "Hans");

        assert!(auth.is_password_correct(&user, "test123").await, "The password should match");
    }

    #[tokio::test]
    async fn should_return_false_when_password_incorrect() {
        let auth = AuthenticationService::new(Arc::new(UserService::new()));

        let user = User::new(1, "test@example.org", "Hans");

        assert!(!auth.is_password_correct(&user, "some123").await, "Password is not correct. This should return false");
    }

}