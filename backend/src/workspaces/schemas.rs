use super::models::Workspace;
use common::api::workspaces::WorkspaceResponse;

impl From<Workspace> for WorkspaceResponse {
    fn from(w: Workspace) -> Self {
        Self {
            id: w.id,
            owner_id: w.owner_id,
            name: w.name,
            description: w.description,
            created_at: w.created_at,
            updated_at: w.updated_at,
        }
    }
}
