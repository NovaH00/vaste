use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct NodesFilter {
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateNodeRequest {
    #[schema(nullable = true)]
    pub parent_id: Option<Uuid>,
    pub name: String,
    #[schema(value_type = Object)]
    pub content: serde_json::Value,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateNodeRequest {
    pub name: String,
    pub position: i32,
    #[schema(value_type = Object)]
    pub content: serde_json::Value,
    #[schema(nullable = true)]
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct NodeResponse {
    pub id: Uuid,
    pub workspace_id: Uuid,
    #[schema(nullable = true)]
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub position: i32,
    #[schema(value_type = Object)]
    pub content: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct MoveNodeRequest {
    #[schema(nullable = true)]
    pub parent_id: Option<Uuid>,
}
