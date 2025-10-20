pub mod child;
pub mod enums;
pub mod response;
pub mod structure;

use bson::DateTime as BsonDateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use child::{Floorplan, FromMap, Portfolio};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(rename = "_id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
    pub user_id: String,
    #[serde(default)]
    pub directory_ids: Vec<String>,
    #[serde(default)]
    pub team_directory_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover_render_image_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_cover_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floorplan_path: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub floorplans: Vec<Floorplan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portfolios: Option<Vec<Portfolio>>,
    #[serde(default)]
    pub portfolio_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_map: Option<FromMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_project_id: Option<String>,
    #[serde(default)]
    pub is_on_air: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<BsonDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<BsonDateTime>,
}

impl Project {
    pub fn mark_created(&mut self) {
        let now = Utc::now();
        let bson_now = BsonDateTime::from_chrono(now);
        self.created_at = Some(bson_now);
        self.updated_at = Some(bson_now);
    }

    pub fn touch(&mut self) {
        self.updated_at = Some(BsonDateTime::from_chrono(Utc::now()));
    }

    pub fn set_default_cover_image(&mut self, value: Option<String>) {
        self.default_cover_image = value
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
    }

    pub fn set_name(&mut self, value: Option<String>) {
        self.name = value
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
    }

    pub fn floorplan_key(&self) -> Option<String> {
        self.id
            .as_ref()
            .map(|id| format!("projects/{}/floorplans.json", id))
    }
}
