mod common;

use axum::http::StatusCode;
use uuid::Uuid;

#[tokio::test]
async fn test_create_workspace() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("ws_create").await;
    let ws_id = app.create_workspace(token, "My Workspace").await;

    assert_ne!(ws_id, Uuid::nil());
}

#[tokio::test]
async fn test_create_workspace_unauthorized() {
    let app = common::TestApp::spawn().await;

    let (status, json) = app
        .send(
            "POST",
            "/api/workspaces",
            None,
            Some(serde_json::json!({
                "name": "Hacked",
                "description": "",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_list_workspaces() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("ws_list").await;

    let ws1 = app.create_workspace(token, "Alpha").await;
    let ws2 = app.create_workspace(token, "Beta").await;

    let (status, json) = app.send("GET", "/api/workspaces", Some(token), None).await;
    assert_eq!(status, StatusCode::OK);
    let list = json.as_array().unwrap();
    assert!(list.iter().any(|w| w["id"] == ws1.to_string()));
    assert!(list.iter().any(|w| w["id"] == ws2.to_string()));
}

#[tokio::test]
async fn test_get_workspace() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("ws_get").await;
    let ws_id = app.create_workspace(token, "Test WS").await;

    let (status, json) = app
        .send(
            "GET",
            &format!("/api/workspaces/{}", ws_id),
            Some(token),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["id"].as_str().unwrap(), ws_id.to_string());
    assert_eq!(json["name"].as_str().unwrap(), "Test WS");
}

#[tokio::test]
async fn test_get_workspace_not_found() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("ws_get_nf").await;

    let (status, json) = app
        .send(
            "GET",
            &format!("/api/workspaces/{}", Uuid::nil()),
            Some(token),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_get_other_users_workspace_returns_not_found() {
    let app = common::TestApp::spawn().await;
    let (_, token1, _, _) = app.register("ws_owner").await;
    let (_, token2, _, _) = app.register("ws_intruder").await;
    let ws_id = app.create_workspace(token1, "Private").await;

    let (status, json) = app
        .send(
            "GET",
            &format!("/api/workspaces/{}", ws_id),
            Some(token2),
            None,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "Should not see others' workspace"
    );
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_update_workspace() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("ws_update").await;
    let ws_id = app.create_workspace(token, "Old Name").await;

    let (status, json) = app
        .send(
            "PATCH",
            &format!("/api/workspaces/{}", ws_id),
            Some(token),
            Some(serde_json::json!({
                "name": "New Name",
                "description": "Updated description",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["name"].as_str().unwrap(), "New Name");
    assert_eq!(json["description"].as_str().unwrap(), "Updated description");
}

#[tokio::test]
async fn test_delete_workspace() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("ws_delete").await;
    let ws_id = app.create_workspace(token, "To Delete").await;

    let (status, _json) = app
        .send(
            "DELETE",
            &format!("/api/workspaces/{}", ws_id),
            Some(token),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    let (status2, _json2) = app
        .send(
            "GET",
            &format!("/api/workspaces/{}", ws_id),
            Some(token),
            None,
        )
        .await;
    assert_eq!(status2, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_other_users_workspace() {
    let app = common::TestApp::spawn().await;
    let (_, token1, _, _) = app.register("ws_owner2").await;
    let (_, token2, _, _) = app.register("ws_intruder2").await;
    let ws_id = app.create_workspace(token1, "Mine").await;

    let (status, _json) = app
        .send(
            "DELETE",
            &format!("/api/workspaces/{}", ws_id),
            Some(token2),
            None,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "Should not delete others' workspace"
    );
}

#[tokio::test]
async fn test_list_workspaces_scoped_to_user() {
    let app = common::TestApp::spawn().await;
    let (_, token1, _, _) = app.register("ws_user1").await;
    let (_, token2, _, _) = app.register("ws_user2").await;

    let ws1 = app.create_workspace(token1, "User1 WS").await;
    app.create_workspace(token2, "User2 WS").await;

    let (status, json) = app.send("GET", "/api/workspaces", Some(token1), None).await;
    assert_eq!(status, StatusCode::OK);
    let ids: Vec<String> = json
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w["id"].as_str().unwrap().to_string())
        .collect();
    assert!(ids.contains(&ws1.to_string()), "Should see own workspace");
    assert_eq!(ids.len(), 1, "Should not see other user's workspace");
}

#[tokio::test]
async fn test_search_workspaces_by_name() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("ws_search").await;

    app.create_workspace(token, "Alpha Project").await;
    app.create_workspace(token, "Beta Project").await;

    let (status, json) = app
        .send("GET", "/api/workspaces?q=Alpha", Some(token), None)
        .await;
    assert_eq!(status, StatusCode::OK);
    let names: Vec<&str> = json
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w["name"].as_str().unwrap())
        .collect();
    assert!(names.contains(&"Alpha Project"));
    assert!(!names.contains(&"Beta Project"));
}

#[tokio::test]
async fn test_search_workspaces_by_description() {
    let app = common::TestApp::spawn().await;
    let (_, token, _, _) = app.register("ws_search_desc").await;

    let (status, _) = app
        .send(
            "POST",
            "/api/workspaces",
            Some(token),
            Some(serde_json::json!({
                "name": "Work",
                "description": "secret stuff here",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);

    let (status, _) = app
        .send(
            "POST",
            "/api/workspaces",
            Some(token),
            Some(serde_json::json!({
                "name": "Play",
                "description": "fun time",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);

    let (status, json) = app
        .send("GET", "/api/workspaces?q=secret", Some(token), None)
        .await;
    assert_eq!(status, StatusCode::OK);
    let names: Vec<&str> = json
        .as_array()
        .unwrap()
        .iter()
        .map(|w| w["name"].as_str().unwrap())
        .collect();
    assert!(names.contains(&"Work"));
    assert!(!names.contains(&"Play"));
}

#[tokio::test]
async fn test_update_workspace_unauthorized() {
    let app = common::TestApp::spawn().await;
    let (_, token1, _, _) = app.register("ws_upd_owner").await;
    let (_, token2, _, _) = app.register("ws_upd_intruder").await;
    let ws_id = app.create_workspace(token1, "Mine").await;

    let (status, json) = app
        .send(
            "PATCH",
            &format!("/api/workspaces/{}", ws_id),
            Some(token2),
            Some(serde_json::json!({
                "name": "Hacked",
                "description": "",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_delete_workspace_unauthorized() {
    let app = common::TestApp::spawn().await;
    let (_, token1, _, _) = app.register("ws_del_owner").await;
    let (_, token2, _, _) = app.register("ws_del_intruder").await;
    let ws_id = app.create_workspace(token1, "Mine").await;

    let (status, json) = app
        .send(
            "DELETE",
            &format!("/api/workspaces/{}", ws_id),
            Some(token2),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(json["error"].as_str().is_some());
}
