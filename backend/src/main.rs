mod agents;
mod auth;
mod chat;
mod config;
mod crypto;
mod error;
mod nodes;
mod users;
mod workspaces;
mod api_doc;

use std::path::Path;

use axum::Router;
use sqlx::PgPool;
use sqlx::migrate::Migrator;
use tokio::net::TcpListener;
use utoipa_swagger_ui::SwaggerUi;
use api_doc::ApiDoc;
use utoipa::OpenApi;


#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_file("Config.toml")?;

    let pool = PgPool::connect(&config.database.get_connection_url()).await?;

    let migrator = Migrator::new(Path::new("migrations")).await?;
    migrator.run(&pool).await?;

    let app_state = AppState { pool };

    let app = Router::new()
        .merge(auth::routers::router())
        .merge(users::routers::router())
        .merge(workspaces::routers::router())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(app_state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Server running on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
