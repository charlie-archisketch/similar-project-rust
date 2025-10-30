use anyhow::anyhow;
use aws_sdk_s3::Client as S3Client;
use reqwest::Client as HttpClient;

use crate::{
    error::ApiError,
    repositories::{
        floor_structure_repository::FloorStructureRepository, image_repository::ImageRepository,
        project_repository::ProjectRepository, room_structure_repository::RoomStructureRepository,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub project_repository: Option<ProjectRepository>,
    pub image_repository: Option<ImageRepository>,
    pub floor_structure_repository: Option<FloorStructureRepository>,
    pub room_structure_repository: Option<RoomStructureRepository>,
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

    pub fn image_repository(&self) -> Result<&ImageRepository, ApiError> {
        self.image_repository
            .as_ref()
            .ok_or_else(|| ApiError::internal(anyhow!("Mongo connection is not configured")))
    }

    pub fn floor_structure_repository(&self) -> Result<&FloorStructureRepository, ApiError> {
        self.floor_structure_repository
            .as_ref()
            .ok_or_else(|| ApiError::internal(anyhow!("Postgres connection is not configured")))
    }

    pub fn room_structure_repository(&self) -> Result<&RoomStructureRepository, ApiError> {
        self.room_structure_repository
            .as_ref()
            .ok_or_else(|| ApiError::internal(anyhow!("Postgres connection is not configured")))
    }
}
