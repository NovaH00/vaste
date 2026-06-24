use super::services::validate_session;
use crate::AppState;
use crate::errors::AppError;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::HeaderMap;
use axum::http::request::Parts;
use uuid::Uuid;

pub fn parse_bearer_token(headers: &HeaderMap) -> Result<Uuid, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("missing or invalid authorization header".into()))?;

    Uuid::parse_str(token).map_err(|_| AppError::Unauthorized("invalid token format".into()))
}

pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        let token = parse_bearer_token(&parts.headers)?;
        let (_, user) = validate_session(&state.pool, token).await?;
        Ok(AuthenticatedUser { user_id: user.id })
    }
}
