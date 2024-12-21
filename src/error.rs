#![allow(dead_code)]

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Static error: {0}")]
    Static(&'static str),

    #[error("Failed to send email: {0}")]
    SendError(#[from] lettre::error::Error),

    #[error("Failed to create email message: {0}")]
    MessageBuildError(String),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
