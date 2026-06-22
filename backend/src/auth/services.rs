use sqlx::PgPool;
use uuid::Uuid;

use crate::crypto;

use super::errors::AuthError;
use super::models::Session;
use super::queries as auth_queries;
use crate::users::models::User;
use crate::users::queries as user_queries;

pub async fn login(
    pool: &PgPool,
    identifier: String,
    password: String,
) -> Result<Session, AuthError> {
    let user = user_queries::get_user_by_email(pool, &identifier)
        .await?
        .or(user_queries::get_user_by_username(pool, &identifier).await?)
        .ok_or(AuthError::InvalidCredentials)?;

    if !crypto::verify_password(&password, &user.password_hash)? {
        return Err(AuthError::InvalidCredentials);
    }

    let session = Session::new(user.id);
    auth_queries::create_session(pool, &session).await?;

    Ok(session)
}

pub async fn logout(pool: &PgPool, token: Uuid) -> Result<(), AuthError> {
    auth_queries::delete_session_by_token(pool, token).await?;
    Ok(())
}

pub async fn validate_session(pool: &PgPool, token: Uuid) -> Result<(Session, User), AuthError> {
    let session = auth_queries::get_session_by_token(pool, token)
        .await?
        .ok_or(AuthError::InvalidToken)?;

    let user = user_queries::get_user(pool, session.user_id)
        .await?
        .ok_or(AuthError::InvalidToken)?; // This shouldn't happen but we handle it gracefully anyway

    if session.is_expired() {
        return Err(AuthError::TokenExpired);
    }

    Ok((session, user))
}

pub async fn refresh_session(pool: &PgPool, token: Uuid) -> Result<Session, AuthError> {
    let (_, user) = validate_session(pool, token).await?;
    auth_queries::delete_session_by_token(pool, token).await?;
    let new_session = Session::new(user.id);
    auth_queries::create_session(pool, &new_session).await?;

    Ok(new_session)
}
