use std::path::Path;

use sqlx::PgPool;
use sqlx::migrate::Migrator;
use tokio::net::TcpListener;

use backend::AppState;
use backend::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_file("Config.toml")?;

    let pool = PgPool::connect(&config.database.get_connection_url()).await?;

    let migrator = Migrator::new(Path::new("migrations")).await?;
    migrator.run(&pool).await?;

    let app = backend::create_app(AppState { pool });

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Server running on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
