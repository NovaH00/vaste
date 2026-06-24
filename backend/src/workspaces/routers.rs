use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

use crate::AppState;
use crate::auth::extractors::AuthenticatedUser;
use crate::errors::AppError;
use crate::workspaces::services;

use common::api::errors::ErrorResponse;
use common::api::workspaces::{
    CreateWorkspaceRequest, UpdateWorkspaceRequest, WorkspaceResponse, WorkspaceSearchQuery,
};

#[utoipa::path(
    post,
    path = "/api/workspaces",
    tag = "workspaces",
    security(("bearer_auth" = [])),
    request_body = CreateWorkspaceRequest,
    responses(
        (status = 201, description = "Workspace created", body = WorkspaceResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn create_workspace(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
    Json(body): Json<CreateWorkspaceRequest>,
) -> Result<(StatusCode, Json<WorkspaceResponse>), AppError> {
    let workspace =
        services::create_workspace(&app_state.pool, user.user_id, body.name, body.description)
            .await?;
    Ok((StatusCode::CREATED, Json(workspace.into())))
}

#[utoipa::path(
    get,
    path = "/api/workspaces",
    tag = "workspaces",
    security(("bearer_auth" = [])),
    params(
        ("q" = Option<String>, Query, description = "Search query"),
    ),
    responses(
        (status = 200, description = "List of workspaces", body = Vec<WorkspaceResponse>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn list_workspaces(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
    Query(params): Query<WorkspaceSearchQuery>,
) -> Result<Json<Vec<WorkspaceResponse>>, AppError> {
    let workspaces = match params.q.filter(|q| !q.is_empty()) {
        Some(query) => services::search_workspaces(&app_state.pool, user.user_id, &query).await?,
        None => services::get_user_workspaces(&app_state.pool, user.user_id).await?,
    };
    Ok(Json(workspaces.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    get,
    path = "/api/workspaces/{id}",
    tag = "workspaces",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Workspace UUID"),
    ),
    responses(
        (status = 200, description = "Workspace found", body = WorkspaceResponse),
        (status = 404, description = "Workspace not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn get_workspace(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<WorkspaceResponse>, AppError> {
    let workspace = services::get_workspace_for_user(&app_state.pool, id, user.user_id).await?;
    Ok(Json(workspace.into()))
}

#[utoipa::path(
    patch,
    path = "/api/workspaces/{id}",
    tag = "workspaces",
    security(("bearer_auth" = [])),
    request_body = UpdateWorkspaceRequest,
    params(
        ("id" = Uuid, Path, description = "Workspace UUID"),
    ),
    responses(
        (status = 200, description = "Workspace updated", body = WorkspaceResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Workspace not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn update_workspace(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateWorkspaceRequest>,
) -> Result<Json<WorkspaceResponse>, AppError> {
    let workspace = services::update_workspace(
        &app_state.pool,
        id,
        user.user_id,
        body.name,
        body.description,
    )
    .await?;
    Ok(Json(workspace.into()))
}

#[utoipa::path(
    delete,
    path = "/api/workspaces/{id}",
    tag = "workspaces",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "Workspace UUID"),
    ),
    responses(
        (status = 200, description = "Workspace deleted"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Workspace not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn delete_workspace(
    user: AuthenticatedUser,
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    services::delete_workspace(&app_state.pool, id, user.user_id).await?;
    Ok(())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/workspaces",
            post(create_workspace).get(list_workspaces),
        )
        .route(
            "/api/workspaces/{id}",
            get(get_workspace)
                .patch(update_workspace)
                .delete(delete_workspace),
        )
}
