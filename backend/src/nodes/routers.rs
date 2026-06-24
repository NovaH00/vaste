use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, patch, post},
};
use uuid::Uuid;

use crate::AppState;
use crate::auth::extractors::AuthenticatedUser;
use crate::errors::AppError;
use crate::nodes::services;
use crate::workspaces;

use common::api::errors::ErrorResponse;
use common::api::nodes::{
    CreateNodeRequest, MoveNodeRequest, NodeResponse, NodesFilter, UpdateNodeRequest,
};

use super::models::NodeContent;

#[utoipa::path(
    post,
    path = "/api/workspaces/{workspace_id}/nodes",
    tag = "nodes",
    security(("bearer_auth" = [])),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace UUID"),
    ),
    request_body = CreateNodeRequest,
    responses(
        (status = 201, description = "Node created", body = NodeResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn create_node(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
    Json(body): Json<CreateNodeRequest>,
) -> Result<(StatusCode, Json<NodeResponse>), AppError> {
    workspaces::services::get_workspace_for_user(&state.pool, workspace_id, user.user_id).await?;

    let content: NodeContent = serde_json::from_value(body.content)
        .map_err(|e| AppError::BadRequest(format!("invalid content: {}", e)))?;

    let node = services::create_node(
        &state.pool,
        workspace_id,
        body.parent_id,
        body.name,
        0,
        content,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(node.into())))
}

#[utoipa::path(
    get,
    path = "/api/workspaces/{workspace_id}/nodes",
    tag = "nodes",
    security(("bearer_auth" = [])),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace UUID"),
        NodesFilter,
    ),
    responses(
        (status = 200, description = "List of nodes", body = Vec<NodeResponse>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn list_nodes(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
    Query(filter): Query<NodesFilter>,
) -> Result<Json<Vec<NodeResponse>>, AppError> {
    workspaces::services::get_workspace_for_user(&state.pool, workspace_id, user.user_id).await?;

    let nodes = match filter.parent_id {
        Some(pid) => services::get_child_nodes(&state.pool, workspace_id, Some(pid)).await?,
        None => services::get_workspace_nodes(&state.pool, workspace_id).await?,
    };

    Ok(Json(nodes.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    get,
    path = "/api/workspaces/{workspace_id}/nodes/{id}",
    tag = "nodes",
    security(("bearer_auth" = [])),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace UUID"),
        ("id" = Uuid, Path, description = "Node UUID"),
    ),
    responses(
        (status = 200, description = "Node found", body = NodeResponse),
        (status = 404, description = "Node not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn get_node(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((workspace_id, id)): Path<(Uuid, Uuid)>,
) -> Result<Json<NodeResponse>, AppError> {
    workspaces::services::get_workspace_for_user(&state.pool, workspace_id, user.user_id).await?;

    let node = services::get_node(&state.pool, id, workspace_id).await?;

    Ok(Json(node.into()))
}

#[utoipa::path(
    patch,
    path = "/api/workspaces/{workspace_id}/nodes/{id}",
    tag = "nodes",
    security(("bearer_auth" = [])),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace UUID"),
        ("id" = Uuid, Path, description = "Node UUID"),
    ),
    request_body = UpdateNodeRequest,
    responses(
        (status = 200, description = "Node updated", body = NodeResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Node not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn update_node(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((workspace_id, id)): Path<(Uuid, Uuid)>,
    Json(body): Json<UpdateNodeRequest>,
) -> Result<Json<NodeResponse>, AppError> {
    workspaces::services::get_workspace_for_user(&state.pool, workspace_id, user.user_id).await?;

    let content: NodeContent = serde_json::from_value(body.content)
        .map_err(|e| AppError::BadRequest(format!("invalid content: {}", e)))?;

    let node = services::update_node(
        &state.pool,
        id,
        workspace_id,
        body.name,
        body.position,
        content,
        body.parent_id,
    )
    .await?;

    Ok(Json(node.into()))
}

#[utoipa::path(
    delete,
    path = "/api/workspaces/{workspace_id}/nodes/{id}",
    tag = "nodes",
    security(("bearer_auth" = [])),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace UUID"),
        ("id" = Uuid, Path, description = "Node UUID"),
    ),
    responses(
        (status = 200, description = "Node deleted"),
        (status = 404, description = "Node not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn delete_node(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((workspace_id, id)): Path<(Uuid, Uuid)>,
) -> Result<(), AppError> {
    workspaces::services::get_workspace_for_user(&state.pool, workspace_id, user.user_id).await?;

    services::remove_node(&state.pool, id, workspace_id).await?;

    Ok(())
}

#[utoipa::path(
    patch,
    path = "/api/workspaces/{workspace_id}/nodes/{id}/move",
    tag = "nodes",
    security(("bearer_auth" = [])),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace UUID"),
        ("id" = Uuid, Path, description = "Node UUID"),
    ),
    request_body = MoveNodeRequest,
    responses(
        (status = 200, description = "Node moved", body = NodeResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Node not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn move_node(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((workspace_id, id)): Path<(Uuid, Uuid)>,
    Json(body): Json<MoveNodeRequest>,
) -> Result<Json<NodeResponse>, AppError> {
    workspaces::services::get_workspace_for_user(&state.pool, workspace_id, user.user_id).await?;

    let node = services::move_node(&state.pool, id, workspace_id, body.parent_id).await?;

    Ok(Json(node.into()))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/workspaces/{workspace_id}/nodes",
            post(create_node).get(list_nodes),
        )
        .route(
            "/api/workspaces/{workspace_id}/nodes/{id}",
            get(get_node).patch(update_node).delete(delete_node),
        )
        .route(
            "/api/workspaces/{workspace_id}/nodes/{id}/move",
            patch(move_node),
        )
}
