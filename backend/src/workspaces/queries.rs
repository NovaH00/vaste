use sqlx::{Executor, Postgres};
use uuid::Uuid;

use super::models::Workspace;

pub async fn create_workspace<'e, E>(executor: E, workspace: &Workspace) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        INSERT INTO workspaces (
            id,
            owner_id,
            name,
            description,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(workspace.id)
    .bind(workspace.owner_id)
    .bind(&workspace.name)
    .bind(&workspace.description)
    .bind(workspace.created_at)
    .bind(workspace.updated_at)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}

pub async fn get_workspace<'e, E>(executor: E, id: Uuid) -> Result<Option<Workspace>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, Workspace>(
        r#"
        SELECT
            id,
            owner_id,
            name,
            description,
            created_at,
            updated_at
        FROM workspaces
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(executor)
    .await
}

pub async fn get_workspaces_by_owner<'e, E>(
    executor: E,
    owner_id: Uuid,
) -> Result<Vec<Workspace>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, Workspace>(
        r#"
        SELECT
            id,
            owner_id,
            name,
            description,
            created_at,
            updated_at
        FROM workspaces
        WHERE owner_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(owner_id)
    .fetch_all(executor)
    .await
}

pub async fn update_workspace<'e, E>(executor: E, workspace: &Workspace) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        UPDATE workspaces
        SET
            name = $2,
            description = $3,
            updated_at = $4
        WHERE id = $1
        "#,
    )
    .bind(workspace.id)
    .bind(&workspace.name)
    .bind(&workspace.description)
    .bind(workspace.updated_at)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}

pub async fn delete_workspace<'e, E>(executor: E, id: Uuid) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        DELETE FROM workspaces
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}

pub async fn search_workspaces<'e, E>(
    executor: E,
    owner_id: Uuid,
    query: &str,
) -> Result<Vec<Workspace>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, Workspace>(
        r#"
        SELECT
            id,
            owner_id,
            name,
            description,
            created_at,
            updated_at
        FROM workspaces
        WHERE owner_id = $1
          AND (name ILIKE $2 OR description ILIKE $2)
        ORDER BY created_at DESC
        "#,
    )
    .bind(owner_id)
    .bind(format!("%{}%", query))
    .fetch_all(executor)
    .await
}
