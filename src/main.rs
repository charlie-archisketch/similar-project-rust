mod models;

use std::net::SocketAddr;

use axum::Router;
use anyhow::Context;
use sea_orm::{Database, DatabaseConnection};
use mongodb::{Client as MongoClient, Database as MongoDatabase};

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
    mongo: Option<MongoDatabase>,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let db_url = std::env::var("DATABASE_URL")
        .context("Missing env var DATABASE_URL (e.g., postgres connection string)")?;
    let port: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);

    let db = Database::connect(&db_url).await?;

    let mongo = match std::env::var("MONGODB_URI") {
        Ok(uri) => {
            let client = MongoClient::with_uri_str(uri).await?;
            let db_name = std::env::var("MONGODB_DB").unwrap_or_else(|_| "ArchisketchDB".to_string());
            let db = client.database(&db_name);
            Some(db)
        }
        Err(_) => None,
    };

    let state = AppState { db, mongo };
    let app = Router::new().with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
