use aws_sdk_s3::Client as S3Client;
use axum::{
    Json,
    extract::{Path, State},
};
use reqwest::{Client as HttpClient, StatusCode};
use std::borrow::ToOwned;

use crate::{
    error::ApiError,
    models::project::{Project, child::floorplan::Floorplan, response::ProjectResponse},
    repositories::project_repository::ProjectRepository,
    state::AppState,
};

pub async fn get_project_by_id(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<ProjectResponse>, ApiError> {
    let repository = state.project_repository()?;
    let mut project = repository.get_by_id(&project_id).await?;
    populate_floorplans(&state.http_client, &state.cdn_base_url, &mut project).await?;
    ensure_default_cover_image(
        repository,
        &mut project,
        state.s3_client.as_ref(),
        state.s3_bucket.as_deref(),
        &state.cdn_base_url,
    )
    .await?;
    let response = ProjectResponse::try_from_project(&project).map_err(ApiError::internal)?;
    Ok(Json(response))
}

async fn populate_floorplans(
    http_client: &HttpClient,
    cdn_base_url: &str,
    project: &mut Project,
) -> Result<(), ApiError> {
    let Some(path) = project.floorplan_path.as_ref() else {
        return Ok(());
    };
    if path.is_empty() {
        return Ok(());
    }

    let Some(key) = project.floorplan_key() else {
        return Ok(());
    };

    let url = format!(
        "{}/{}",
        cdn_base_url.trim_end_matches('/'),
        key.trim_start_matches('/')
    );
    let response = http_client
        .get(&url)
        .send()
        .await
        .map_err(ApiError::internal)?;

    if response.status() == StatusCode::NOT_FOUND {
        return Ok(());
    }
    if !response.status().is_success() {
        return Err(ApiError::internal(anyhow::anyhow!(
            "failed to fetch floorplans from {url}: status {}",
            response.status()
        )));
    }

    let payload = response.text().await.map_err(ApiError::internal)?;
    let floorplans: Vec<Floorplan> = serde_json::from_str(&payload).map_err(ApiError::internal)?;
    project.floorplans = floorplans;
    Ok(())
}

async fn ensure_default_cover_image(
    repository: &ProjectRepository,
    project: &mut Project,
    s3_client: Option<&S3Client>,
    s3_bucket: Option<&str>,
    cdn_base_url: &str,
) -> Result<(), ApiError> {
    if project.default_cover_image.is_some() {
        return Ok(());
    }

    let cover_image = project
        .cover_image
        .clone()
        .ok_or_else(|| ApiError::internal(anyhow::anyhow!("project missing cover_image")))?;

    if let (Some(client), Some(bucket), Some(project_id)) =
        (s3_client, s3_bucket, project.id.clone())
    {
        let prefix = format!("projects/{project_id}/images");
        let response = client
            .list_objects_v2()
            .bucket(bucket)
            .prefix(&prefix)
            .send()
            .await
            .map_err(ApiError::internal)?;

        let contents = response.contents();
        if !contents.is_empty() {
            let mut latest_key = None;
            let mut latest_time = None;

            for object in contents {
                let key = object.key().map(ToOwned::to_owned);
                let modified = object.last_modified().cloned();
                match (key, modified) {
                    (Some(key), Some(modified)) => {
                        if latest_time.map(|t| t < modified).unwrap_or(true) {
                            latest_time = Some(modified);
                            latest_key = Some(key);
                        }
                    }
                    _ => continue,
                }
            }

            if let Some(key) = latest_key {
                let url = format!(
                    "{}/{}",
                    cdn_base_url.trim_end_matches('/'),
                    key.trim_start_matches('/')
                );
                project.default_cover_image = Some(url.clone());
                repository
                    .persist_default_cover_image(&project_id, &url)
                    .await?;
            }
        }
    }

    if project.default_cover_image.is_none() {
        project.default_cover_image = Some(cover_image);
        if let Some(project_id) = project.id.clone() {
            if let Some(default_image) = project.default_cover_image.clone() {
                repository
                    .persist_default_cover_image(&project_id, &default_image)
                    .await?;
            }
        }
    }

    Ok(())
}
