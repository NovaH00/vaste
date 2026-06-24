mod common;

use axum::http::StatusCode;
use uuid::Uuid;

#[tokio::test]
async fn test_create_folder_node() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_create").await;
    let workspace_id = app.create_workspace(token, "Node Test WS").await;

    let (status, json) = app
        .send(
            "POST",
            &format!("/api/workspaces/{}/nodes", workspace_id),
            Some(token),
            Some(serde_json::json!({
                "name": "My Folder",
                "parent_id": null,
                "content": { "type": "folder" },
            })),
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);
    assert!(json["id"].as_str().is_some());
    assert_eq!(json["name"].as_str().unwrap(), "My Folder");
    assert_eq!(json["content"]["type"].as_str().unwrap(), "folder");
}

#[tokio::test]
async fn test_create_text_node() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_text").await;
    let workspace_id = app.create_workspace(token, "Text WS").await;

    let (status, json) = app
        .send(
            "POST",
            &format!("/api/workspaces/{}/nodes", workspace_id),
            Some(token),
            Some(serde_json::json!({
                "name": "Readme",
                "parent_id": null,
                "content": { "type": "text", "body": "# Hello" },
            })),
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(json["content"]["type"].as_str().unwrap(), "text");
    assert_eq!(json["content"]["body"].as_str().unwrap(), "# Hello");
}

#[tokio::test]
async fn test_list_root_nodes() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_list").await;
    let workspace_id = app.create_workspace(token, "List WS").await;

    app.create_folder_node(token, workspace_id, "Folder A")
        .await;
    app.create_folder_node(token, workspace_id, "Folder B")
        .await;

    let (status, json) = app
        .send(
            "GET",
            &format!("/api/workspaces/{}/nodes", workspace_id),
            Some(token),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    let nodes = json.as_array().unwrap();
    assert_eq!(nodes.len(), 2);
}

#[tokio::test]
async fn test_get_node() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_get").await;
    let workspace_id = app.create_workspace(token, "Get WS").await;
    let node_id = app.create_folder_node(token, workspace_id, "Target").await;

    let (status, json) = app
        .send(
            "GET",
            &format!("/api/workspaces/{}/nodes/{}", workspace_id, node_id),
            Some(token),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["id"].as_str().unwrap(), node_id.to_string());
    assert_eq!(json["name"].as_str().unwrap(), "Target");
}

#[tokio::test]
async fn test_get_node_not_found() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_get_nf").await;
    let workspace_id = app.create_workspace(token, "NF WS").await;

    let (status, json) = app
        .send(
            "GET",
            &format!("/api/workspaces/{}/nodes/{}", workspace_id, Uuid::nil()),
            Some(token),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_update_node() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_upd").await;
    let workspace_id = app.create_workspace(token, "Upd WS").await;
    let node_id = app
        .create_folder_node(token, workspace_id, "Old Name")
        .await;

    let (status, json) = app
        .send(
            "PATCH",
            &format!("/api/workspaces/{}/nodes/{}", workspace_id, node_id),
            Some(token),
            Some(serde_json::json!({
                "name": "New Name",
                "position": 1,
                "content": { "type": "folder" },
                "parent_id": null,
            })),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["name"].as_str().unwrap(), "New Name");
    assert_eq!(json["position"], 1);
}

#[tokio::test]
async fn test_delete_node() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_del").await;
    let workspace_id = app.create_workspace(token, "Del WS").await;
    let node_id = app
        .create_folder_node(token, workspace_id, "To Delete")
        .await;

    let (status, _json) = app
        .send(
            "DELETE",
            &format!("/api/workspaces/{}/nodes/{}", workspace_id, node_id),
            Some(token),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    let (status2, _json2) = app
        .send(
            "GET",
            &format!("/api/workspaces/{}/nodes/{}", workspace_id, node_id),
            Some(token),
            None,
        )
        .await;
    assert_eq!(status2, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_move_node() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_move").await;
    let workspace_id = app.create_workspace(token, "Move WS").await;

    let parent_id = app.create_folder_node(token, workspace_id, "Parent").await;
    let child_id = app.create_folder_node(token, workspace_id, "Child").await;

    let (status, json) = app
        .send(
            "PATCH",
            &format!("/api/workspaces/{}/nodes/{}/move", workspace_id, child_id),
            Some(token),
            Some(serde_json::json!({
                "parent_id": parent_id.to_string(),
            })),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["parent_id"].as_str().unwrap(), parent_id.to_string());
}

#[tokio::test]
async fn test_move_node_to_itself_returns_bad_request() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_move_self").await;
    let workspace_id = app.create_workspace(token, "MoveSelf WS").await;
    let node_id = app.create_folder_node(token, workspace_id, "Lonely").await;

    let (status, json) = app
        .send(
            "PATCH",
            &format!("/api/workspaces/{}/nodes/{}/move", workspace_id, node_id),
            Some(token),
            Some(serde_json::json!({
                "parent_id": node_id.to_string(),
            })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_nodes_scoped_to_workspace() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_scope").await;
    let ws1 = app.create_workspace(token, "WS1").await;
    let ws2 = app.create_workspace(token, "WS2").await;

    app.create_folder_node(token, ws1, "In WS1").await;
    app.create_folder_node(token, ws2, "In WS2").await;

    let (status, json) = app
        .send(
            "GET",
            &format!("/api/workspaces/{}/nodes", ws1),
            Some(token),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    let names: Vec<&str> = json
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n["name"].as_str().unwrap())
        .collect();
    assert!(names.contains(&"In WS1"));
    assert!(!names.contains(&"In WS2"));
}

#[tokio::test]
async fn test_child_nodes_are_deleted_with_parent() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_cascade").await;
    let workspace_id = app.create_workspace(token, "Cascade WS").await;

    let parent_id = app.create_folder_node(token, workspace_id, "Parent").await;
    let child_id = app.create_folder_node(token, workspace_id, "Child").await;

    // Move child into parent
    let _ = app
        .send(
            "PATCH",
            &format!("/api/workspaces/{}/nodes/{}/move", workspace_id, child_id),
            Some(token),
            Some(serde_json::json!({ "parent_id": parent_id.to_string() })),
        )
        .await;

    // Delete parent (should cascade)
    let (del_status, _) = app
        .send(
            "DELETE",
            &format!("/api/workspaces/{}/nodes/{}", workspace_id, parent_id),
            Some(token),
            None,
        )
        .await;
    assert_eq!(del_status, StatusCode::OK);

    // Child should also be deleted
    let (get_status, _) = app
        .send(
            "GET",
            &format!("/api/workspaces/{}/nodes/{}", workspace_id, child_id),
            Some(token),
            None,
        )
        .await;
    assert_eq!(
        get_status,
        StatusCode::NOT_FOUND,
        "Child should be cascade-deleted"
    );
}

#[tokio::test]
async fn test_create_node_unauthorized() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_cr_unauth").await;
    let ws_id = app.create_workspace(token, "WS").await;

    let (status, json) = app
        .send(
            "POST",
            &format!("/api/workspaces/{}/nodes", ws_id),
            None,
            Some(serde_json::json!({
                "name": "Hacked",
                "parent_id": null,
                "content": { "type": "folder" },
            })),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_get_node_unauthorized() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_get_unauth").await;
    let ws_id = app.create_workspace(token, "WS").await;
    let node_id = app.create_folder_node(token, ws_id, "Secret").await;

    let (status, json) = app
        .send(
            "GET",
            &format!("/api/workspaces/{}/nodes/{}", ws_id, node_id),
            None,
            None,
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_update_node_unauthorized() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_upd_unauth").await;
    let ws_id = app.create_workspace(token, "WS").await;
    let node_id = app.create_folder_node(token, ws_id, "Target").await;

    let (status, json) = app
        .send(
            "PATCH",
            &format!("/api/workspaces/{}/nodes/{}", ws_id, node_id),
            None,
            Some(serde_json::json!({
                "name": "Hacked",
                "position": 0,
                "content": { "type": "folder" },
                "parent_id": null,
            })),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_delete_node_unauthorized() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_del_unauth").await;
    let ws_id = app.create_workspace(token, "WS").await;
    let node_id = app.create_folder_node(token, ws_id, "Target").await;

    let (status, json) = app
        .send(
            "DELETE",
            &format!("/api/workspaces/{}/nodes/{}", ws_id, node_id),
            None,
            None,
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_get_node_from_wrong_workspace() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("node_wr_ws").await;
    let ws1 = app.create_workspace(token, "WS1").await;
    let ws2 = app.create_workspace(token, "WS2").await;
    let node_id = app.create_folder_node(token, ws1, "In WS1").await;

    let (status, json) = app
        .send(
            "GET",
            &format!("/api/workspaces/{}/nodes/{}", ws2, node_id),
            Some(token),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(json["error"].as_str().is_some());
}
