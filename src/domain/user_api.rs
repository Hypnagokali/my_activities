use async_trait::async_trait;

use crate::{domain::user::User, error::errors::{QueryUserError, UserUpdateError}};

use super::user::Credentials;

#[async_trait]
pub trait UserApi: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<User, QueryUserError>;
    async fn find_by_id(&self, user_id: i32) -> Result<User, QueryUserError>;
    async fn save_user_with_credentials(&self, user: User, password: &str) -> Result<User, UserUpdateError>;
    async fn save_credentials(&self, credentials: Credentials) -> Result<Credentials, UserUpdateError>;
    async fn find_credentials_by_user_id(&self, user_id: i32) -> Result<Credentials, QueryUserError>;
}

