use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Node {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub position: i32,
    pub content: Json<NodeContent>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeContent {
    #[serde(rename = "folder")]
    Folder,

    #[serde(rename = "text")]
    MarkdownFile { body: String },

    #[serde(rename = "todo_board")]
    TodoBoard { items: Vec<TodoItem> },

    #[serde(rename = "note_board")]
    NoteBoard { sticky_notes: Vec<StickyNote> },

    #[serde(rename = "schedule")]
    Schedule {
        monday: Vec<ScheduleEvent>,
        tuesday: Vec<ScheduleEvent>,
        wednesday: Vec<ScheduleEvent>,
        thursday: Vec<ScheduleEvent>,
        friday: Vec<ScheduleEvent>,
        saturday: Vec<ScheduleEvent>,
        sunday: Vec<ScheduleEvent>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: Uuid,
    pub title: String,
    pub done: bool,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickyNote {
    pub id: Uuid,
    pub content: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleEvent {
    pub id: Uuid,
    pub title: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub description: String,
}

impl Node {
    pub fn new(
        workspace_id: Uuid,
        parent_id: Option<Uuid>,
        name: String,
        position: i32,
        content: NodeContent,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::now_v7(),
            workspace_id,
            parent_id,
            name,
            position,
            content: Json(content),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    pub fn set_position(&mut self, position: i32) {
        self.position = position;
        self.updated_at = Utc::now();
    }

    pub fn set_content(&mut self, content: NodeContent) {
        self.content = Json(content);
        self.updated_at = Utc::now();
    }

    pub fn set_parent(&mut self, parent_id: Option<Uuid>) {
        self.parent_id = parent_id;
        self.updated_at = Utc::now();
    }
}
