use sqlx::PgPool;
use uuid::Uuid;

use super::errors::NodeServiceError;
use super::models::{Node, NodeContent};
use super::queries;

pub async fn get_node(
    pool: &PgPool,
    node_id: Uuid,
    workspace_id: Uuid,
) -> Result<Node, NodeServiceError> {
    let node = queries::get_node(pool, node_id)
        .await?
        .ok_or(NodeServiceError::NotFound)?;

    if node.workspace_id != workspace_id {
        return Err(NodeServiceError::NotInWorkspace);
    }

    Ok(node)
}

pub async fn get_workspace_nodes(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<Node>, NodeServiceError> {
    Ok(queries::get_nodes_by_workspace(pool, workspace_id).await?)
}

pub async fn get_child_nodes(
    pool: &PgPool,
    workspace_id: Uuid,
    parent_id: Option<Uuid>,
) -> Result<Vec<Node>, NodeServiceError> {
    Ok(queries::get_nodes_by_parent(pool, workspace_id, parent_id).await?)
}

pub async fn create_node(
    pool: &PgPool,
    workspace_id: Uuid,
    parent_id: Option<Uuid>,
    name: String,
    position: i32,
    content: NodeContent,
) -> Result<Node, NodeServiceError> {
    let node = Node::new(workspace_id, parent_id, name, position, content);
    queries::create_node(pool, &node).await?;
    Ok(node)
}

pub async fn update_node(
    pool: &PgPool,
    node_id: Uuid,
    workspace_id: Uuid,
    name: String,
    position: i32,
    content: NodeContent,
    parent_id: Option<Uuid>,
) -> Result<Node, NodeServiceError> {
    let mut node = get_node(pool, node_id, workspace_id).await?;

    if parent_id != node.parent_id {
        if let Some(target) = parent_id {
            if queries::is_descendant_of(pool, target, node_id).await? {
                return Err(NodeServiceError::CircularParent);
            }
        }
    }

    node.set_name(name);
    node.set_position(position);
    node.set_content(content);
    node.set_parent(parent_id);
    queries::update_node(pool, &node).await?;

    Ok(node)
}

pub async fn remove_node(
    pool: &PgPool,
    node_id: Uuid,
    workspace_id: Uuid,
) -> Result<(), NodeServiceError> {
    let _node = get_node(pool, node_id, workspace_id).await?;
    queries::delete_node_cascade(pool, node_id).await?;
    Ok(())
}

pub async fn move_node(
    pool: &PgPool,
    node_id: Uuid,
    workspace_id: Uuid,
    target_parent_id: Option<Uuid>,
) -> Result<Node, NodeServiceError> {
    let mut node = get_node(pool, node_id, workspace_id).await?;

    if let Some(target) = target_parent_id {
        if target == node_id {
            return Err(NodeServiceError::CircularParent);
        }
        if queries::is_descendant_of(pool, target, node_id).await? {
            return Err(NodeServiceError::CircularParent);
        }
    }

    node.set_parent(target_parent_id);
    queries::update_node(pool, &node).await?;

    Ok(node)
}
