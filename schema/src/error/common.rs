use uuid::Uuid;
use async_graphql::{ErrorExtensions, FieldError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("Element not found in the database")]
    NotFound(Uuid),
    #[error("Invalid request")]
    InvalidRequest(String),
    #[error("Internal Error")]
    InternalError(String),

}

impl ErrorExtensions for CommonError {
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            Self::NotFound(id) => {
                e.set("code", "NOT_FOUND");
                e.set("message", format!("Element not found with id : {}", id))
            },
            Self::InternalError(message) => {
                e.set("code", "INTERNAL_ERROR");
                e.set("message", message.to_string())
            },
            Self::InvalidRequest(message) => {
                e.set("code", "INVALID_REQUEST");
                e.set("message", message.to_string())
            }
        })
    }
}