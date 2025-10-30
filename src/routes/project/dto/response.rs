use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

use crate::models::{image::Image as ProjectImage, project::Project};
use crate::utils::image::convert_image_url;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
    #[serde(default)]
    pub directory_ids: Vec<String>,
    #[serde(default)]
    pub team_directory_ids: Vec<String>,
    pub name: String,
    pub cover_image: String,
    pub default_cover_image: String,
    pub floorplan_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    #[serde(default)]
    pub bookmark: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl ProjectResponse {
    pub fn try_from_project(project: &Project) -> Result<Self> {
        let id = project
            .id
            .clone()
            .ok_or_else(|| anyhow!("project missing _id"))?;
        let cover_image_raw = project
            .cover_image
            .clone()
            .ok_or_else(|| anyhow!("project missing cover_image"))?;
        let default_cover_image_raw = project
            .default_cover_image
            .clone()
            .ok_or_else(|| anyhow!("project missing default_cover_image"))?;
        let floorplan_path = project
            .floorplan_path
            .clone()
            .ok_or_else(|| anyhow!("project missing floorplan_path"))?;
        let name = project.name.clone().unwrap_or_default();

        let cover_image = convert_image_url(&cover_image_raw, 512);
        let default_cover_image = convert_image_url(&default_cover_image_raw, 512);
        let created_at = project.created_at.as_ref().map(|dt| dt.to_chrono());
        let updated_at = project.updated_at.as_ref().map(|dt| dt.to_chrono());

        Ok(Self {
            id,
            user_id: project.user_id.clone(),
            enterprise_id: project.enterprise_id.clone(),
            directory_ids: project.directory_ids.clone(),
            team_directory_ids: project.team_directory_ids.clone(),
            name,
            cover_image,
            default_cover_image,
            floorplan_path,
            state: project.state,
            bookmark: false,
            created_at,
            updated_at,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomResponse {
    pub id: String,
    pub project_id: String,
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_cover_image: Option<String>,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl RoomResponse {
    pub fn try_from_project(project: &Project, id: &str) -> Result<Self> {
        let project_id = project
            .id
            .clone()
            .ok_or_else(|| anyhow!("missing project id"))?;
        let created_at = project.created_at.as_ref().map(|dt| dt.to_chrono());
        let updated_at = project.updated_at.as_ref().map(|dt| dt.to_chrono());

        Ok(Self {
            id: id.to_string(),
            project_id,
            project_name: project.name.clone().unwrap_or_default(),
            cover_image: project.cover_image.clone(),
            default_cover_image: project.default_cover_image.clone(),
            user_id: project.user_id.clone(),
            state: project.state,
            created_at,
            updated_at,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FloorResponse {
    pub id: String,
    pub title: String,
    pub project_id: String,
    pub project_name: String,
    pub user_id: String,
    pub area: f64,
    #[serde(default)]
    pub image_urls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_cover_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl FloorResponse {
    pub fn try_from_project(
        project: &Project,
        id: &str,
        title: &str,
        cdn_base_url: &str,
        area: f64,
        image_map: &HashMap<String, ProjectImage>,
    ) -> Result<Self> {
        let project_id = project
            .id
            .clone()
            .ok_or_else(|| anyhow!("missing project id"))?;
        let created_at = project.created_at.as_ref().map(|dt| dt.to_chrono());
        let updated_at = project.updated_at.as_ref().map(|dt| dt.to_chrono());
        let image_urls = project
            .image_ids
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|image_id| {
                image_map.get(&image_id).map(|image| {
                    format!(
                        "{}/images/{image_id}/{}x{}/{image_id}.png",
                        cdn_base_url.trim_end_matches('/'),
                        image.resolution.x,
                        image.resolution.y
                    )
                })
            })
            .collect();

        Ok(Self {
            id: id.to_string(),
            title: title.to_string(),
            project_id,
            project_name: project.name.clone().unwrap_or_default(),
            user_id: project.user_id.clone(),
            area,
            image_urls,
            cover_image: project.cover_image.clone(),
            default_cover_image: project.default_cover_image.clone(),
            state: project.state,
            created_at,
            updated_at,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRenderingsResponse {
    pub images: Vec<ProjectRenderingImageResponse>,
}

impl ProjectRenderingsResponse {
    pub fn new(images: Vec<ProjectRenderingImageResponse>) -> Self {
        Self { images }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRenderingImageResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: i32,
    pub status: i32,
    pub resolution: ProjectRenderingResolutionResponse,
}

impl From<&ProjectImage> for ProjectRenderingImageResponse {
    fn from(image: &ProjectImage) -> Self {
        Self {
            id: image.id.clone(),
            r#type: image.image_type,
            status: image.status,
            resolution: ProjectRenderingResolutionResponse {
                x: image.resolution.x,
                y: image.resolution.y,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProjectRenderingResolutionResponse {
    pub x: i32,
    pub y: i32,
}
