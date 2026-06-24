use sqlx::{Executor, Postgres};
use uuid::Uuid;

use super::models::Node;

pub async fn get_node<'e, E>(executor: E, id: Uuid) -> Result<Option<Node>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, Node>(
        r#"
        SELECT
            id,
            workspace_id,
            parent_id,
            name,
            position,
            content,
            created_at,
            updated_at
        FROM nodes
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(executor)
    .await
}

pub async fn get_nodes_by_workspace<'e, E>(
    executor: E,
    workspace_id: Uuid,
) -> Result<Vec<Node>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, Node>(
        r#"
        SELECT
            id,
            workspace_id,
            parent_id,
            name,
            position,
            content,
            created_at,
            updated_at
        FROM nodes
        WHERE workspace_id = $1
        ORDER BY position, created_at
        "#,
    )
    .bind(workspace_id)
    .fetch_all(executor)
    .await
}

pub async fn get_nodes_by_parent<'e, E>(
    executor: E,
    workspace_id: Uuid,
    parent_id: Option<Uuid>,
) -> Result<Vec<Node>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    match parent_id {
        Some(pid) => {
            sqlx::query_as::<_, Node>(
                r#"
                SELECT
                    id,
                    workspace_id,
                    parent_id,
                    name,
                    position,
                    content,
                    created_at,
                    updated_at
                FROM nodes
                WHERE workspace_id = $1 AND parent_id = $2
                ORDER BY position, created_at
                "#,
            )
            .bind(workspace_id)
            .bind(pid)
            .fetch_all(executor)
            .await
        }
        None => {
            sqlx::query_as::<_, Node>(
                r#"
                SELECT
                    id,
                    workspace_id,
                    parent_id,
                    name,
                    position,
                    content,
                    created_at,
                    updated_at
                FROM nodes
                WHERE workspace_id = $1 AND parent_id IS NULL
                ORDER BY position, created_at
                "#,
            )
            .bind(workspace_id)
            .fetch_all(executor)
            .await
        }
    }
}

pub async fn create_node<'e, E>(executor: E, node: &Node) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        INSERT INTO nodes (id, workspace_id, parent_id, name, position, content, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(node.id)
    .bind(node.workspace_id)
    .bind(node.parent_id)
    .bind(&node.name)
    .bind(node.position)
    .bind(&node.content)
    .bind(node.created_at)
    .bind(node.updated_at)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}

pub async fn update_node<'e, E>(executor: E, node: &Node) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        UPDATE nodes
        SET
            name = $2,
            position = $3,
            content = $4,
            parent_id = $5,
            updated_at = $6
        WHERE id = $1
        "#,
    )
    .bind(node.id)
    .bind(&node.name)
    .bind(node.position)
    .bind(&node.content)
    .bind(node.parent_id)
    .bind(node.updated_at)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}

pub async fn delete_node<'e, E>(executor: E, id: Uuid) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        DELETE FROM nodes
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}

pub async fn delete_node_cascade<'e, E>(executor: E, id: Uuid) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        WITH RECURSIVE descendants AS (
            SELECT id FROM nodes WHERE id = $1
            UNION ALL
            SELECT n.id FROM nodes n INNER JOIN descendants d ON n.parent_id = d.id
        )
        DELETE FROM nodes WHERE id IN (SELECT id FROM descendants)
        "#,
    )
    .bind(id)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}

pub async fn is_descendant_of<'e, E>(
    executor: E,
    node_id: Uuid,
    potential_ancestor_id: Uuid,
) -> Result<bool, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result: Option<bool> = sqlx::query_scalar(
        r#"
        WITH RECURSIVE ancestors AS (
            SELECT parent_id FROM nodes WHERE id = $1
            UNION ALL
            SELECT n.parent_id FROM nodes n INNER JOIN ancestors a ON n.id = a.parent_id
        )
        SELECT EXISTS(SELECT 1 FROM ancestors WHERE parent_id = $2)
        "#,
    )
    .bind(node_id)
    .bind(potential_ancestor_id)
    .fetch_optional(executor)
    .await?;

    Ok(result.unwrap_or(false))
}
