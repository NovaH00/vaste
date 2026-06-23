use axum::{Json, Router, extract::State, http::HeaderMap, routing::post};

use super::extractors::parse_bearer_token;
use super::services;
use crate::AppState;
use crate::error::AppError;
use common::api::auth::{LoginRequest, LoginResponse};
use common::api::error::ErrorResponse;


#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
async fn login(
    State(app_state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let session = services::login(&app_state.pool, body.identifier, body.password).await?;
    Ok(Json(LoginResponse {
        token: session.token,
        user_id: session.user_id,
        expires_at: session.expires_at,
    }))
}


#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "auth",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Logged out successfully"),
        (status = 401, description = "Invalid or missing token", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
async fn logout(State(app_state): State<AppState>, headers: HeaderMap) -> Result<(), AppError> {
    let token = parse_bearer_token(&headers)?;
    services::logout(&app_state.pool, token).await?;
    Ok(())
}


#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = "auth",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Token refreshed", body = LoginResponse),
        (status = 401, description = "Invalid or expired token", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
async fn refresh(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<LoginResponse>, AppError> {
    let token = parse_bearer_token(&headers)?;
    let session = services::refresh_session(&app_state.pool, token).await?;
    Ok(Json(LoginResponse {
        token: session.token,
        user_id: session.user_id,
        expires_at: session.expires_at,
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/refresh", post(refresh))
}
