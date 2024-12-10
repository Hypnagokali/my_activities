use crate::{domain::user::User, error::errors::NotFoundError};

pub trait UserApi {
    fn find_by_email(&self, email: &str) -> Result<User, NotFoundError>;
}

