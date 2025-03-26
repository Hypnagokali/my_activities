use async_trait::async_trait;

use super::user::User;

#[async_trait]
pub trait AuthenticationApi: Send + Sync {
    async fn is_password_correct(&self, user: &User, password: &str) -> bool; 
}