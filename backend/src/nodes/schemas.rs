use super::models::Node;
use common::api::nodes::NodeResponse;

impl From<Node> for NodeResponse {
    fn from(node: Node) -> Self {
        Self {
            id: node.id,
            workspace_id: node.workspace_id,
            parent_id: node.parent_id,
            name: node.name,
            position: node.position,
            content: serde_json::to_value(node.content.0).unwrap_or_default(),
            created_at: node.created_at,
            updated_at: node.updated_at,
        }
    }
}
