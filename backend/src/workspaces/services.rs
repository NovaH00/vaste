use sqlx::PgPool;
use uuid::Uuid;

use super::errors::WorkspaceServiceError;
use super::models::Workspace;
use super::queries;

pub async fn create_workspace(
    pool: &PgPool,
    owner_id: Uuid,
    name: String,
    description: Option<String>,
) -> Result<Workspace, WorkspaceServiceError> {
    let workspace = Workspace::new(owner_id, name, description);
    queries::create_workspace(pool, &workspace).await?;
    Ok(workspace)
}

pub async fn get_workspace_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Workspace, WorkspaceServiceError> {
    queries::get_workspace(pool, id)
        .await?
        .ok_or(WorkspaceServiceError::WorkspaceNotFound)
}

pub async fn get_workspace_for_user(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<Workspace, WorkspaceServiceError> {
    let workspace = get_workspace_by_id(pool, id).await?;
    if workspace.owner_id != user_id {
        return Err(WorkspaceServiceError::NotOwner);
    }
    Ok(workspace)
}

pub async fn get_user_workspaces(
    pool: &PgPool,
    owner_id: Uuid,
) -> Result<Vec<Workspace>, WorkspaceServiceError> {
    let workspaces = queries::get_workspaces_by_owner(pool, owner_id).await?;
    Ok(workspaces)
}

pub async fn update_workspace(
    pool: &PgPool,
    id: Uuid,
    owner_id: Uuid,
    name: String,
    description: Option<String>,
) -> Result<Workspace, WorkspaceServiceError> {
    let mut workspace = queries::get_workspace(pool, id)
        .await?
        .ok_or(WorkspaceServiceError::WorkspaceNotFound)?;

    if workspace.owner_id != owner_id {
        return Err(WorkspaceServiceError::NotOwner);
    }

    workspace.set_name(name);
    workspace.set_description(description);
    queries::update_workspace(pool, &workspace).await?;

    Ok(workspace)
}

pub async fn delete_workspace(
    pool: &PgPool,
    id: Uuid,
    owner_id: Uuid,
) -> Result<(), WorkspaceServiceError> {
    let workspace = queries::get_workspace(pool, id)
        .await?
        .ok_or(WorkspaceServiceError::WorkspaceNotFound)?;

    if workspace.owner_id != owner_id {
        return Err(WorkspaceServiceError::NotOwner);
    }

    queries::delete_workspace(pool, id).await?;
    Ok(())
}

pub async fn search_workspaces(
    pool: &PgPool,
    owner_id: Uuid,
    query: &str,
) -> Result<Vec<Workspace>, WorkspaceServiceError> {
    Ok(queries::search_workspaces(pool, owner_id, query).await?)
}
