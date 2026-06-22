use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Workspace {
    pub id: Uuid,
    pub owner_id: Uuid,

    pub name: String,
    pub description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Workspace {
    pub fn new(owner_id: Uuid, name: String, description: Option<String>) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::now_v7(),
            owner_id,
            name,
            description,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }
}
