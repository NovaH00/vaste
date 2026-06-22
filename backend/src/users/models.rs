use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,

    pub display_name: String,
    pub bio: String,

    pub email: String,
    pub username: String,
    pub password_hash: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        display_name: String,
        bio: String,
        email: String,
        username: String,
        password_hash: String,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::now_v7(),
            display_name,
            bio,
            email,
            username,
            password_hash,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
        self.updated_at = Utc::now();
    }

    pub fn set_bio(&mut self, bio: String) {
        self.bio = bio;
        self.updated_at = Utc::now();
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
        self.updated_at = Utc::now();
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
        self.updated_at = Utc::now();
    }

    pub fn set_password_hash(&mut self, password_hash: String) {
        self.password_hash = password_hash;
        self.updated_at = Utc::now();
    }
}
