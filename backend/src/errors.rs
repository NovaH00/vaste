use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use common::api::errors::ErrorResponse;

use crate::auth::errors::AuthError;
use crate::nodes::errors::NodeServiceError;
use crate::users::errors::UserServiceError;
use crate::workspaces::errors::WorkspaceServiceError;

#[derive(Debug)]
pub enum AppError {
    Unauthorized(String),
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized(m) => (StatusCode::UNAUTHORIZED, m),
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };
        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::InvalidCredentials => AppError::Unauthorized("invalid credentials".into()),
            AuthError::InvalidToken => AppError::Unauthorized("invalid token".into()),
            AuthError::TokenExpired => AppError::Unauthorized("token expired".into()),
            AuthError::SessionNotFound => AppError::NotFound("session not found".into()),
            AuthError::Database(e) => AppError::Internal(e.to_string()),
            AuthError::Password(e) => AppError::Internal(e.to_string()),
        }
    }
}

impl From<UserServiceError> for AppError {
    fn from(err: UserServiceError) -> Self {
        use AppError as AE;
        use UserServiceError as USE;
        match err {
            USE::UserNotFound => AE::NotFound("user not found".into()),
            USE::EmailAlreadyExists => AE::BadRequest("email already exists".into()),
            USE::UsernameAlreadyExists => AE::BadRequest("username already exists".into()),
            USE::InvalidCredentials => AE::Unauthorized("invalid credentials".into()),
            USE::InvalidEmail(msg) => AE::BadRequest(msg),
            USE::InvalidUsername(msg) => AE::BadRequest(msg),
            USE::WeakPassword(msg) => AE::BadRequest(msg),
            USE::InvalidPassword => AE::Unauthorized("invalid password".into()),
            USE::PasswordHash(e) => AE::Internal(e.to_string()),
            USE::Database(e) => AE::Internal(e.to_string()),
        }
    }
}

impl From<WorkspaceServiceError> for AppError {
    fn from(err: WorkspaceServiceError) -> Self {
        use AppError as AE;
        use WorkspaceServiceError as WSE;
        match err {
            WSE::WorkspaceNotFound => AE::NotFound("workspace not found".into()),
            WSE::NotOwner => AE::NotFound("workspace not found".into()),
            WSE::Database(e) => AE::Internal(e.to_string()),
        }
    }
}

impl From<NodeServiceError> for AppError {
    fn from(err: NodeServiceError) -> Self {
        use AppError as AE;
        use NodeServiceError as NSE;
        match err {
            NSE::NotFound => AE::NotFound("node not found".into()),
            NSE::NotInWorkspace => AE::NotFound("node not found in workspace".into()),
            NSE::CircularParent => {
                AE::BadRequest("cannot move node into its own descendant".into())
            }
            NSE::Database(e) => AE::Internal(e.to_string()),
        }
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}
