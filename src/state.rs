use anyhow::anyhow;
use aws_sdk_s3::Client as S3Client;
use reqwest::Client as HttpClient;
use crate::{error::ApiError, repositories::project_repository::ProjectRepository};

#[derive(Clone)]
pub struct AppState {
    pub project_repository: Option<ProjectRepository>,
    pub s3_client: Option<S3Client>,
    pub s3_bucket: Option<String>,
    pub cdn_base_url: String,
    pub http_client: HttpClient,
}

impl AppState {
    pub fn project_repository(&self) -> Result<&ProjectRepository, ApiError> {
        self.project_repository
            .as_ref()
            .ok_or_else(|| ApiError::internal(anyhow!("Mongo connection is not configured")))
    }
}
