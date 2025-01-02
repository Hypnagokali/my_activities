use async_trait::async_trait;

use super::user::User;

pub trait AuthToken {
    fn is_authenticated(&self) -> bool;
    fn get_authenticated_user(&self) -> Result<User, ()>;
}

#[async_trait]
pub trait AuthenticationApi: Send + Sync {
    async fn is_password_correct(&self, user: &User, password: &str) -> bool; 
    fn is_authenticated(&self, auth: &dyn AuthToken) -> bool;
    fn get_authenticated_user(&self, auth: &dyn AuthToken) -> Result<User, ()>;
}