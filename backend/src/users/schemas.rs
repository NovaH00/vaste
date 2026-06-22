use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::User;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct RegisterRequest {
    pub display_name: String,
    pub bio: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub display_name: String,
    pub bio: String,
    pub email: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            display_name: user.display_name,
            bio: user.bio,
            email: user.email,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateUsernameRequest {
    pub new_username: String,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateEmailRequest {
    pub new_email: String,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateProfileRequest {
    pub display_name: String,
    pub bio: String,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}
