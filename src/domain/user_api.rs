use async_trait::async_trait;

use crate::{domain::user::User, error::errors::NotFoundError};

#[async_trait]
pub trait UserApi: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<User, NotFoundError>;
}

