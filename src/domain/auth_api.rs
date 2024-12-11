use super::user::User;

pub trait AuthenticationApi {
    fn is_password_correct(&self, user: &User, password: &str) -> bool; 
}