mod config;
mod error;
mod handlers;
mod models;
mod repositories;
mod routes;
mod state;
mod utils;

use std::net::SocketAddr;

use aws_config::{BehaviorVersion, meta::region::RegionProviderChain};
use aws_sdk_s3::Client as S3Client;
use aws_types::region::Region;
use axum::Router;
use mongodb::Client as MongoClient;
use reqwest::Client as HttpClient;
use routes::app_router;
use sea_orm::Database;
use state::AppState;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "trace".into()))
        .with_target(false)
        .compact()
        .init();

    let config = config::AppConfig::load()?;

    let mongo = if let Some(uri) = &config.mongodb_uri {
        let client = MongoClient::with_uri_str(uri).await?;
        Some(client.database(&config.mongodb_db))
    } else {
        None
    };

    let postgres = if let Some(url) = &config.database_url {
        Some(Database::connect(url).await?)
    } else {
        None
    };

    let s3_client = if config.s3_bucket_name.is_some() {
        let region_provider = if let Some(region) = &config.aws_region {
            RegionProviderChain::first_try(Region::new(region.clone())).or_default_provider()
        } else {
            RegionProviderChain::default_provider()
        };
        let aws_conf = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;
        Some(S3Client::new(&aws_conf))
    } else {
        None
    };

    let project_repository = mongo
        .as_ref()
        .map(repositories::project_repository::ProjectRepository::new);
    let floor_structure_repository = postgres
        .clone()
        .map(repositories::floor_structure_repository::FloorStructureRepository::new);
    let room_structure_repository =
        postgres.map(repositories::room_structure_repository::RoomStructureRepository::new);
    let http_client = HttpClient::new();

    let state: AppState = AppState {
        project_repository,
        floor_structure_repository,
        room_structure_repository,
        s3_client,
        s3_bucket: config.s3_bucket_name.clone(),
        cdn_base_url: config.cdn_url.clone(),
        http_client,
    };
    let router: Router<_> = app_router().with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
