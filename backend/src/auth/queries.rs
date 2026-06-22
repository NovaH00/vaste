use sqlx::{Executor, Postgres};
use uuid::Uuid;

use super::models::Session;

pub async fn create_session<'e, E>(executor: E, session: &Session) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        INSERT INTO sessions (token, user_id, expires_at, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(session.token)
    .bind(session.user_id)
    .bind(session.expires_at)
    .bind(session.created_at)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}

pub async fn get_session_by_token<'e, E>(
    executor: E,
    token: Uuid,
) -> Result<Option<Session>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, Session>(
        r#"
        SELECT token, user_id, expires_at, created_at
        FROM sessions
        WHERE token = $1
        "#,
    )
    .bind(token)
    .fetch_optional(executor)
    .await
}

pub async fn delete_session_by_token<'e, E>(executor: E, token: Uuid) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        DELETE FROM sessions
        WHERE token = $1
        "#,
    )
    .bind(token)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}

pub async fn delete_user_sessions<'e, E>(executor: E, user_id: Uuid) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        DELETE FROM sessions
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}
