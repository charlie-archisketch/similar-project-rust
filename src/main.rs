use std::net::SocketAddr;

use axum::Router;
use anyhow::Context;
use tracing_subscriber::EnvFilter;
use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("debug"))
        .with_target(false)
        .with_writer(std::io::stdout)
        .compact()
        .init();

    let db_url = std::env::var("DATABASE_URL")
        .context("Missing env var DATABASE_URL (e.g., postgres connection string)")?;
    let port: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);

    let db = Database::connect(&db_url).await?;

    let state = AppState { db };
    let app = Router::new().with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
