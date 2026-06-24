mod common;

use axum::http::StatusCode;

#[tokio::test]
async fn test_login_success() {
    let app = common::TestApp::spawn().await;
    let (_user_id, _token, email, password) = app.register("login_success").await;

    let (status, json) = app.login(&email, &password).await;
    assert_eq!(status, StatusCode::OK);
    assert!(json["token"].as_str().is_some());
    assert!(json["user_id"].as_str().is_some());
    assert!(json["expires_at"].as_str().is_some());
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let app = common::TestApp::spawn().await;

    let (status, json) = app.login("nonexistent@test.com", "wrongpass").await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_login_wrong_password() {
    let app = common::TestApp::spawn().await;
    let (_user_id, _token, email, _password) = app.register("login_wrong_pw").await;

    let (status, json) = app.login(&email, "WrongPassword999!").await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_logout_success() {
    let app = common::TestApp::spawn().await;
    let (_user_id, token, _, _) = app.register("logout_success").await;

    let (status, _json) = app
        .send("POST", "/api/auth/logout", Some(token), None)
        .await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_logout_without_token() {
    let app = common::TestApp::spawn().await;

    let (status, json) = app.send("POST", "/api/auth/logout", None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_refresh_token() {
    let app = common::TestApp::spawn().await;
    let (_user_id, token, _, _) = app.register("refresh_success").await;

    let (status, json) = app
        .send("POST", "/api/auth/refresh", Some(token), None)
        .await;
    assert_eq!(status, StatusCode::OK);
    assert!(json["token"].as_str().is_some());
    assert!(json["user_id"].as_str().is_some());
}

#[tokio::test]
async fn test_refresh_expired_token() {
    let app = common::TestApp::spawn().await;

    let (status, json) = app
        .send("POST", "/api/auth/refresh", Some(uuid::Uuid::nil()), None)
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_login_twice_returns_different_tokens() {
    let app = common::TestApp::spawn().await;
    let (_user_id, token1, email, password) = app.register("login_twice").await;

    let (status, json) = app.login(&email, &password).await;
    assert_eq!(status, StatusCode::OK);
    let token2 = json["token"].as_str().unwrap().to_string();

    assert_ne!(
        token1.to_string(),
        token2,
        "Second login should issue a new token"
    );
}

#[tokio::test]
async fn test_login_with_username() {
    let app = common::TestApp::spawn().await;
    let suffix = common::TestApp::unique_suffix();
    let username = format!("login_user_{}", suffix);
    let password = "Password123!";

    let (status, _) = app
        .send(
            "POST",
            "/api/users",
            None,
            Some(serde_json::json!({
                "username": username,
                "email": format!("{}@test.com", suffix),
                "password": password,
                "display_name": "login user",
                "bio": "",
            })),
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);

    let (status, json) = app.login(&username, password).await;
    assert_eq!(status, StatusCode::OK);
    assert!(json["token"].as_str().is_some());
}

#[tokio::test]
async fn test_logout_then_refresh_fails() {
    let app = common::TestApp::spawn().await;
    let (_user_id, token, _, _) = app.register("logout_reuse").await;

    let (status, _) = app.send("POST", "/api/auth/logout", Some(token), None).await;
    assert_eq!(status, StatusCode::OK);

    // Refresh with logged-out token should fail
    let (status, _) = app.send("POST", "/api/auth/refresh", Some(token), None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
