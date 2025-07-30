use std::sync::Arc;

use actix_web::HttpRequest;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use authfix::{login::LoadUserByCredentials, multifactor::config::{HandleMfaRequest, MfaError}};
use crate::{domain::{auth_api::AuthenticationApi, user::User, user_api::UserApi}, error::errors::QueryUserError};

pub struct AuthenticationService<U: UserApi> {
    user_api: Arc<U>
}

impl<U: UserApi> AuthenticationService<U> {
    pub fn new(user_api: Arc<U>) -> Self {
        AuthenticationService {
            user_api,
        }
    }
}

#[async_trait]
impl<U: UserApi> AuthenticationApi for AuthenticationService<U> {
    async fn is_password_correct(&self, user: &User, password: &str) -> bool {
        println!("Check if password correct!");
        match self.user_api.find_credentials_by_user_id(user.id).await {
            Ok(credentials) => {
                let argon2 = Argon2::default();
                match PasswordHash::new(&credentials.password)  {
                    Ok(hash) => {
                        match argon2.verify_password(password.as_bytes(), &hash) {
                            Ok(_) => true,
                            Err(_) => false,
                        }
                    },
                    Err(_) => {
                        log::error!("Could not create PasswordHash from credentials");
                        false
                    },
                }
            },
            Err(_) => false,
        }
    }
}

impl<U: UserApi> LoadUserByCredentials for AuthenticationService<U> {
    type User = User;

    async fn load_user(
        &self,
        login_token: &authfix::login::LoginToken,
    ) -> Result<Self::User, authfix::login::LoadUserError> {
        let email = login_token.email.clone();
        let password = login_token.password.clone();
        
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
    }
}


pub struct HandleMfaRequestImpl<S> {
    user_api: Arc<S>,
}

impl<S: UserApi> HandleMfaRequestImpl<S> {
    pub fn new(user_api: Arc<S>) -> Self {
        Self {
            user_api,
        }
    }
}

#[async_trait(?Send)]
impl<S: UserApi> HandleMfaRequest for HandleMfaRequestImpl<S> {
    type User = User;

    async fn mfa_id_by_user(&self, user: &Self::User) -> Result<Option<String>, MfaError> {
        let creds = self.user_api.find_credentials_by_user_id(user.id).await?;
        if let Some(mfa_config) = creds.mfa_config {
            Ok(Some(mfa_config.mfa_id))
        } else {
            Ok(None)
        }
    }

    #[allow(unused)]
    async fn is_condition_met(&self, user: &Self::User, req: HttpRequest) -> bool {
        match self.user_api.find_credentials_by_user_id(user.id).await {
            Ok(creds) => creds.mfa_config.is_some(),
            Err(_) => false,
        }
    }
}

impl From<QueryUserError> for MfaError {
    fn from(value: QueryUserError) -> Self {
        MfaError::new(&format!("User query error: {}", &value))
    }
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{config::db::DbConfig, create_db, domain::{auth_api::AuthenticationApi, user::User, user_api::UserApi}, service::user_service::UserService};

    use super::AuthenticationService;


    #[tokio::test]
    async fn should_return_true_when_password_correct() {
        // Arrange
        let db_config = DbConfig::new(":memory");
        create_db(&db_config);
        let user_service = Arc::new(UserService::new(Arc::new(db_config)));
        let auth = AuthenticationService::new(Arc::clone(&user_service));
        let user = User::new(0, "test@example.org".to_owned(), "Hans".to_owned());
        let saved_user = user_service.save_user_with_credentials(user, "test123").await.unwrap();

        // Act & Assert 
        assert!(auth.is_password_correct(&saved_user, "test123").await, "The password should match");
    }

    #[tokio::test]
    async fn should_return_false_when_password_incorrect() {
        let db_config = DbConfig::new(":memory");
        create_db(&db_config);
        let user_service = Arc::new(UserService::new(Arc::new(db_config)));
        let auth = AuthenticationService::new(Arc::clone(&user_service));
        let user = User::new(0, "test@example.org".to_owned(), "Hans".to_owned());
        let saved_user = user_service.save_user_with_credentials(user, "test123").await.unwrap();

        assert!(!auth.is_password_correct(&saved_user, "some123").await, "Password is not correct. This should return false");
    }

}