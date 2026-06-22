use sqlx::PgPool;
use uuid::Uuid;

use crate::crypto;

use super::errors::UserServiceError;
use super::models::User;
use super::queries;
use super::validation;

pub async fn register_user(
    pool: &PgPool,
    display_name: String,
    bio: String,
    email: String,
    username: String,
    password: String,
) -> Result<User, UserServiceError> {
    validation::validate_email(&email)?;
    validation::validate_username(&username)?;
    validation::validate_password(&password)?;

    if queries::get_user_by_email(pool, &email).await?.is_some() {
        return Err(UserServiceError::EmailAlreadyExists);
    }

    if queries::get_user_by_username(pool, &username)
        .await?
        .is_some()
    {
        return Err(UserServiceError::UsernameAlreadyExists);
    }

    let password_hash = crypto::hash_password(&password)?;
    let new_user = User::new(display_name, bio, email, username, password_hash);
    queries::create_user(pool, &new_user).await?;

    Ok(new_user)
}

pub async fn get_user_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<User, UserServiceError> {
    queries::get_user(pool, user_id)
        .await?
        .ok_or(UserServiceError::UserNotFound)
}

pub async fn change_username(
    pool: &PgPool,
    user_id: Uuid,
    new_username: String,
) -> Result<User, UserServiceError> {
    validation::validate_username(&new_username)?;

    if queries::get_user_by_username(pool, &new_username)
        .await?
        .is_some()
    {
        return Err(UserServiceError::UsernameAlreadyExists);
    }
    let mut user = get_user_by_user_id(pool, user_id).await?;

    user.set_username(new_username);
    queries::update_user(pool, &user).await?;

    Ok(user)
}

pub async fn change_email(
    pool: &PgPool,
    user_id: Uuid,
    new_email: String,
) -> Result<User, UserServiceError> {
    validation::validate_email(&new_email)?;

    if queries::get_user_by_email(pool, &new_email)
        .await?
        .is_some()
    {
        return Err(UserServiceError::EmailAlreadyExists);
    }

    let mut user = get_user_by_user_id(pool, user_id).await?;

    user.set_email(new_email);
    queries::update_user(pool, &user).await?;

    Ok(user)
}

pub async fn change_profile(
    pool: &PgPool,
    user_id: Uuid,
    display_name: String,
    bio: String,
) -> Result<User, UserServiceError> {
    let mut user = get_user_by_user_id(pool, user_id).await?;

    user.set_display_name(display_name);
    user.set_bio(bio);
    queries::update_user(pool, &user).await?;

    Ok(user)
}

pub async fn change_password(
    pool: &PgPool,
    user_id: Uuid,
    old_password: String,
    new_password: String,
) -> Result<User, UserServiceError> {
    validation::validate_password(&new_password)?;

    let mut user = get_user_by_user_id(pool, user_id).await?;

    if !crypto::verify_password(&old_password, &user.password_hash)? {
        return Err(UserServiceError::InvalidPassword);
    }

    let new_hash = crypto::hash_password(&new_password)?;
    user.set_password_hash(new_hash);
    queries::update_user(pool, &user).await?;

    Ok(user)
}

pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<User, UserServiceError> {
    let user = get_user_by_user_id(pool, user_id).await?;

    queries::delete_user(pool, user_id).await?;

    Ok(user)
}
