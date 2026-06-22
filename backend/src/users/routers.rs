use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch, post},
};
use uuid::Uuid;

use crate::auth::extractors::AuthenticatedUser;
use crate::error::AppError;
use crate::AppState;
use crate::users::services;

use common::error::ErrorResponse;
use super::schemas::{
    ChangePasswordRequest, RegisterRequest, UpdateEmailRequest,
    UpdateProfileRequest, UpdateUsernameRequest, UserResponse,
};

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered", body = UserResponse),
        (status = 400, description = "Validation error or duplicate", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "users",
)]
async fn register(
    State(app_state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let user = services::register_user(
        &app_state.pool,
        body.display_name,
        body.bio,
        body.email,
        body.username,
        body.password,
    )
    .await?;
    Ok((StatusCode::CREATED, Json(user.into())))
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User UUID"),
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "users",
)]
async fn get_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = services::get_user_by_user_id(&app_state.pool, user_id).await?;
    Ok(Json(user.into()))
}

#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "users",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Current user profile", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn get_me(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
) -> Result<Json<UserResponse>, AppError> {
    let user = services::get_user_by_user_id(&app_state.pool, user.user_id).await?;
    Ok(Json(user.into()))
}

#[utoipa::path(
    patch,
    path = "/api/users/me/username",
    tag = "users",
    security(("bearer_auth" = [])),
    request_body = UpdateUsernameRequest,
    responses(
        (status = 200, description = "Username updated", body = UserResponse),
        (status = 400, description = "Invalid or duplicate username", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn update_username(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
    Json(body): Json<UpdateUsernameRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = services::change_username(&app_state.pool, user.user_id, body.new_username).await?;
    Ok(Json(user.into()))
}

#[utoipa::path(
    patch,
    path = "/api/users/me/email",
    tag = "users",
    security(("bearer_auth" = [])),
    request_body = UpdateEmailRequest,
    responses(
        (status = 200, description = "Email updated", body = UserResponse),
        (status = 400, description = "Invalid or duplicate email", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn update_email(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
    Json(body): Json<UpdateEmailRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = services::change_email(&app_state.pool, user.user_id, body.new_email).await?;
    Ok(Json(user.into()))
}

#[utoipa::path(
    patch,
    path = "/api/users/me/profile",
    tag = "users",
    security(("bearer_auth" = [])),
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn update_profile(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
    Json(body): Json<UpdateProfileRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = services::change_profile(
        &app_state.pool,
        user.user_id,
        body.display_name,
        body.bio,
    )
    .await?;
    Ok(Json(user.into()))
}

#[utoipa::path(
    patch,
    path = "/api/users/me/password",
    tag = "users",
    security(("bearer_auth" = [])),
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed", body = UserResponse),
        (status = 400, description = "Invalid password", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn change_password(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
    Json(body): Json<ChangePasswordRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = services::change_password(
        &app_state.pool,
        user.user_id,
        body.old_password,
        body.new_password,
    )
    .await?;
    Ok(Json(user.into()))
}

#[utoipa::path(
    delete,
    path = "/api/users/me",
    tag = "users",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "User deleted"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn delete_user_handler(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
) -> Result<(), AppError> {
    services::delete_user(&app_state.pool, user.user_id).await?;
    Ok(())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/users", post(register))
        .route("/api/users/{id}", get(get_user))
        .route("/api/users/me", get(get_me).delete(delete_user_handler))
        .route("/api/users/me/username", patch(update_username))
        .route("/api/users/me/email", patch(update_email))
        .route("/api/users/me/profile", patch(update_profile))
        .route("/api/users/me/password", patch(change_password))
}
