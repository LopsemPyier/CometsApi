use async_graphql::{ErrorExtensions, FieldError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid Username")]
    InvalidUsername,
    #[error("Invalid Password")]
    InvalidPassword,
    #[error("Email already taken")]
    EmailTaken
}

impl ErrorExtensions for AuthError {
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            Self::InvalidPassword => {
                e.set("code", "INVALID_PASSWORD");
            },
            Self::InvalidUsername => {
                e.set("code", "INVALID_USERNAME");
            },
            Self::EmailTaken => {
                e.set("code", "EMAIL_TAKEN");
            }
        })
    }
}