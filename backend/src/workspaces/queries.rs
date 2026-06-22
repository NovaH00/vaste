use sqlx::{Executor, Postgres};
use uuid::Uuid;

use super::models::Workspace;

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
