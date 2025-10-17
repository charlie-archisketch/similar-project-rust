use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::project::Project;
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
