#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("invalid token")]
    InvalidToken,

    #[error("token expired")]
    TokenExpired,

    #[error("session not found")]
    SessionNotFound,

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Password(#[from] argon2::password_hash::Error),
}
