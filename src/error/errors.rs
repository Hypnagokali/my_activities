use core::fmt;

#[derive(Debug)]
pub struct NotFoundError {
    pub message: String,
}

impl NotFoundError {
    pub fn new(msg: &str) -> Self {
        NotFoundError {
            message: msg.to_owned(),
        }
    }
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for NotFoundError {}

