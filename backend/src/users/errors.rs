use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("user not found")]
    UserNotFound,

    #[error("email already exists")]
    EmailAlreadyExists,

    #[error("username already exists")]
    UsernameAlreadyExists,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("{0}")]
    InvalidEmail(String),

    #[error("{0}")]
    InvalidUsername(String),

    #[error("{0}")]
    WeakPassword(String),

    #[error("invalid password")]
    InvalidPassword,

    #[error(transparent)]
    PasswordHash(#[from] argon2::password_hash::Error),

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}
