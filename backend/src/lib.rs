pub mod agents;
pub mod api_doc;
pub mod auth;
pub mod chat;
pub mod config;
pub mod crypto;
pub mod errors;
pub mod nodes;
pub mod users;
pub mod workspaces;

use axum::Router;
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .merge(auth::routers::router())
        .merge(users::routers::router())
        .merge(workspaces::routers::router())
        .merge(nodes::routers::router())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api_doc::ApiDoc::openapi()))
        .with_state(state)
}
