mod common;

use axum::http::StatusCode;
use uuid::Uuid;

#[tokio::test]
async fn test_register_success() {
    let app = common::TestApp::spawn().await;
    let (user_id, token, _, _) = app.register("reg_success").await;

    assert_ne!(user_id, Uuid::nil());
    assert_ne!(token, Uuid::nil());
}

#[tokio::test]
async fn test_register_duplicate_email() {
    let app = common::TestApp::spawn().await;
    let email = format!("{}@test.com", common::TestApp::unique_suffix());
    let password = "Pass1234!";

    let (status, _json) = app
        .send(
            "POST",
            "/api/users",
            None,
            Some(serde_json::json!({
                "username": format!("user_{}", common::TestApp::unique_suffix()),
                "email": email,
                "password": password,
                "display_name": "first",
                "bio": "",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);

    let (status, json) = app
        .send(
            "POST",
            "/api/users",
            None,
            Some(serde_json::json!({
                "username": format!("user_{}", common::TestApp::unique_suffix()),
                "email": email,
                "password": password,
                "display_name": "second",
                "bio": "",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_register_invalid_email() {
    let app = common::TestApp::spawn().await;

    let (status, json) = app
        .send(
            "POST",
            "/api/users",
            None,
            Some(serde_json::json!({
                "username": "testuser",
                "email": "not-an-email",
                "password": "Pass1234!",
                "display_name": "test",
                "bio": "",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_register_weak_password() {
    let app = common::TestApp::spawn().await;

    let (status, json) = app
        .send(
            "POST",
            "/api/users",
            None,
            Some(serde_json::json!({
                "username": "testuser",
                "email": format!("{}@test.com", common::TestApp::unique_suffix()),
                "password": "short",
                "display_name": "test",
                "bio": "",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_get_user_by_id() {
    let app = common::TestApp::spawn().await;
    let (user_id, _token, _, _) = app.register("get_user").await;

    let (status, json) = app
        .send("GET", &format!("/api/users/{}", user_id), None, None)
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["id"].as_str().unwrap(), user_id.to_string());
}

#[tokio::test]
async fn test_get_user_not_found() {
    let app = common::TestApp::spawn().await;
    let fake_id = Uuid::nil();

    let (status, json) = app
        .send("GET", &format!("/api/users/{}", fake_id), None, None)
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_get_me() {
    let app = common::TestApp::spawn().await;
    let (user_id, token, _, _) = app.register("get_me").await;

    let (status, json) = app.send("GET", "/api/users/me", Some(token), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["id"].as_str().unwrap(), user_id.to_string());
}

#[tokio::test]
async fn test_get_me_unauthorized() {
    let app = common::TestApp::spawn().await;

    let (status, json) = app.send("GET", "/api/users/me", None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_update_username() {
    let app = common::TestApp::spawn().await;
    let (user_id, token, _, _) = app.register("upd_uname").await;
    let new_username = format!("new_{}", common::TestApp::unique_suffix());

    let (status, json) = app
        .send(
            "PATCH",
            "/api/users/me/username",
            Some(token),
            Some(serde_json::json!({ "new_username": new_username })),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["username"].as_str().unwrap(), new_username);

    let (status2, json2) = app
        .send("GET", &format!("/api/users/{}", user_id), None, None)
        .await;
    assert_eq!(status2, StatusCode::OK);
    assert_eq!(json2["username"].as_str().unwrap(), new_username);
}

#[tokio::test]
async fn test_update_email() {
    let app = common::TestApp::spawn().await;
    let (user_id, token, _, _) = app.register("upd_email").await;
    let new_email = format!("{}@test.com", common::TestApp::unique_suffix());

    let (status, json) = app
        .send(
            "PATCH",
            "/api/users/me/email",
            Some(token),
            Some(serde_json::json!({ "new_email": new_email })),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["email"].as_str().unwrap(), new_email);

    let (status2, json2) = app
        .send("GET", &format!("/api/users/{}", user_id), None, None)
        .await;
    assert_eq!(status2, StatusCode::OK);
    assert_eq!(json2["email"].as_str().unwrap(), new_email);
}

#[tokio::test]
async fn test_update_profile() {
    let app = common::TestApp::spawn().await;
    let (user_id, token, _, _) = app.register("upd_profile").await;

    let (status, json) = app
        .send(
            "PATCH",
            "/api/users/me/profile",
            Some(token),
            Some(serde_json::json!({
                "display_name": "Updated Name",
                "bio": "Updated bio",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["display_name"].as_str().unwrap(), "Updated Name");
    assert_eq!(json["bio"].as_str().unwrap(), "Updated bio");

    let (status2, json2) = app
        .send("GET", &format!("/api/users/{}", user_id), None, None)
        .await;
    assert_eq!(status2, StatusCode::OK);
    assert_eq!(json2["display_name"].as_str().unwrap(), "Updated Name");
}

#[tokio::test]
async fn test_change_password() {
    let app = common::TestApp::spawn().await;
    let (_user_id, token, email, old_password) = app.register("chg_pw").await;
    let new_password = "NewPass5678!";

    let (status, _json) = app
        .send(
            "PATCH",
            "/api/users/me/password",
            Some(token),
            Some(serde_json::json!({
                "old_password": old_password,
                "new_password": new_password,
            })),
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    let (status2, _json2) = app.login(&email, new_password).await;
    assert_eq!(status2, StatusCode::OK, "Should login with new password");

    let (status3, _json3) = app.login(&email, &old_password).await;
    assert_eq!(
        status3,
        StatusCode::UNAUTHORIZED,
        "Old password should fail"
    );
}

#[tokio::test]
async fn test_delete_user() {
    let app = common::TestApp::spawn().await;
    let (user_id, token, _, _) = app.register("del_user").await;

    let (status, _json) = app.send("DELETE", "/api/users/me", Some(token), None).await;
    assert_eq!(status, StatusCode::OK);

    let (status2, _json2) = app
        .send("GET", &format!("/api/users/{}", user_id), None, None)
        .await;
    assert_eq!(status2, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_user_unauthorized() {
    let app = common::TestApp::spawn().await;

    let (status, json) = app.send("DELETE", "/api/users/me", None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_register_duplicate_username() {
    let app = common::TestApp::spawn().await;
    let suffix = common::TestApp::unique_suffix();
    let username = format!("dup_user_{}", suffix);
    let password = "Pass1234!";

    let (status, _) = app
        .send(
            "POST",
            "/api/users",
            None,
            Some(serde_json::json!({
                "username": &username,
                "email": format!("{}_first@test.com", suffix),
                "password": password,
                "display_name": "first",
                "bio": "",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);

    let (status, json) = app
        .send(
            "POST",
            "/api/users",
            None,
            Some(serde_json::json!({
                "username": &username,
                "email": format!("{}_second@test.com", suffix),
                "password": password,
                "display_name": "second",
                "bio": "",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_change_password_wrong_old_password() {
    let app = common::TestApp::spawn().await;
    let (_user_id, token, _, _) = app.register("chg_pw_wrong").await;

    let (status, json) = app
        .send(
            "PATCH",
            "/api/users/me/password",
            Some(token),
            Some(serde_json::json!({
                "old_password": "WrongPassword999!",
                "new_password": "NewPass5678!",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(json["error"].as_str().is_some());
}
