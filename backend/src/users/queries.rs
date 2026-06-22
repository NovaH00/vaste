use sqlx::{Executor, Postgres};
use uuid::Uuid;

use super::models::User;

pub async fn get_user<'e, E>(executor: E, id: Uuid) -> Result<Option<User>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, User>(
        r#"
        SELECT
            id,
            display_name,
            bio,
            email,
            username,
            password_hash,
            created_at,
            updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(executor)
    .await
}

pub async fn get_user_by_email<'e, E>(executor: E, email: &str) -> Result<Option<User>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, User>(
        r#"
        SELECT
            id,
            display_name,
            bio,
            email,
            username,
            password_hash,
            created_at,
            updated_at
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(executor)
    .await
}

pub async fn get_user_by_username<'e, E>(
    executor: E,
    username: &str,
) -> Result<Option<User>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, User>(
        r#"
        SELECT
            id,
            display_name,
            bio,
            email,
            username,
            password_hash,
            created_at,
            updated_at
        FROM users
        WHERE username = $1
        "#,
    )
    .bind(username)
    .fetch_optional(executor)
    .await
}

pub async fn create_user<'e, E>(executor: E, user: &User) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        INSERT INTO users (
            id,
            display_name,
            bio,
            email,
            username,
            password_hash,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(user.id)
    .bind(&user.display_name)
    .bind(&user.bio)
    .bind(&user.email)
    .bind(&user.username)
    .bind(&user.password_hash)
    .bind(user.created_at)
    .bind(user.updated_at)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}

pub async fn update_user<'e, E>(executor: E, user: &User) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        UPDATE users
        SET
            display_name = $2,
            bio = $3,
            email = $4,
            username = $5,
            password_hash = $6,
            updated_at = $7
        WHERE id = $1
        "#,
    )
    .bind(user.id)
    .bind(&user.display_name)
    .bind(&user.bio)
    .bind(&user.email)
    .bind(&user.username)
    .bind(&user.password_hash)
    .bind(user.updated_at)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}

pub async fn delete_user<'e, E>(executor: E, id: Uuid) -> Result<u64, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(executor)
    .await?;

    Ok(result.rows_affected())
}
