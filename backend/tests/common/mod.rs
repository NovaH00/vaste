use std::path::Path;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use sqlx::PgPool;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use backend::AppState;
use backend::config::Config;

pub struct TestApp {
    pool: PgPool,
}

#[allow(dead_code)]
impl TestApp {
    pub async fn spawn() -> Self {
        let config = Config::from_file("Config.toml").unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&config.database.get_connection_url())
            .await
            .expect("Failed to connect to database");

        let migrator = Migrator::new(Path::new("migrations")).await.unwrap();
        migrator.run(&pool).await.unwrap();

        sqlx::query("BEGIN").execute(&pool).await.unwrap();

        TestApp { pool }
    }

    pub async fn request(&self, req: Request<Body>) -> (StatusCode, serde_json::Value) {
        let app = backend::create_app(AppState {
            pool: self.pool.clone(),
        });

        let response = tower::ServiceExt::oneshot(app, req)
            .await
            .expect("Request failed");

        let status = response.status();
        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value =
            serde_json::from_slice(&body_bytes).unwrap_or(serde_json::Value::Null);

        (status, json)
    }

    pub async fn send(
        &self,
        method: &str,
        uri: &str,
        token: Option<Uuid>,
        body: Option<serde_json::Value>,
    ) -> (StatusCode, serde_json::Value) {
        let mut builder = Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json");

        if let Some(token) = token {
            builder = builder.header("authorization", format!("Bearer {}", token));
        }

        let req = match body {
            Some(b) => builder
                .body(Body::from(serde_json::to_string(&b).unwrap()))
                .unwrap(),
            None => builder.body(Body::empty()).unwrap(),
        };

        self.request(req).await
    }

    pub fn unique_suffix() -> String {
        (Uuid::now_v7().as_u128() as u32).to_string()
    }

    pub async fn register(&self, tag: &str) -> (Uuid, Uuid, String, String) {
        let suffix = Self::unique_suffix();
        let email = format!("{}@test.com", suffix);
        let password = "Password123!".to_string();
        let (status, json) = self
            .send(
                "POST",
                "/api/users",
                None,
                Some(serde_json::json!({
                    "username": format!("{}_{}", tag, suffix),
                    "email": email,
                    "password": password,
                    "display_name": tag,
                    "bio": "",
                })),
            )
            .await;

        assert_eq!(status, StatusCode::CREATED, "Register failed: {:#?}", json);

        let user_id: Uuid = json["id"].as_str().unwrap().parse().unwrap();
        let email = json["email"].as_str().unwrap().to_string();

        let (_status, login_json) = self.login(&email, &password).await;
        let token: Uuid = login_json["token"].as_str().unwrap().parse().unwrap();
        (user_id, token, email, password)
    }

    pub async fn login(&self, identifier: &str, password: &str) -> (StatusCode, serde_json::Value) {
        self.send(
            "POST",
            "/api/auth/login",
            None,
            Some(serde_json::json!({
                "identifier": identifier,
                "password": password,
            })),
        )
        .await
    }

    pub async fn create_workspace(&self, token: Uuid, name: &str) -> Uuid {
        let (status, json) = self
            .send(
                "POST",
                "/api/workspaces",
                Some(token),
                Some(serde_json::json!({
                    "name": name,
                    "description": "test workspace",
                })),
            )
            .await;

        assert_eq!(
            status,
            StatusCode::CREATED,
            "Create workspace failed: {:#?}",
            json
        );
        json["id"].as_str().unwrap().parse().unwrap()
    }

    pub async fn create_folder_node(&self, token: Uuid, workspace_id: Uuid, name: &str) -> Uuid {
        let uri = format!("/api/workspaces/{}/nodes", workspace_id);
        let (status, json) = self
            .send(
                "POST",
                &uri,
                Some(token),
                Some(serde_json::json!({
                    "name": name,
                    "parent_id": null,
                    "content": { "type": "folder" },
                })),
            )
            .await;

        assert_eq!(
            status,
            StatusCode::CREATED,
            "Create node failed: {:#?}",
            json
        );
        json["id"].as_str().unwrap().parse().unwrap()
    }
}
