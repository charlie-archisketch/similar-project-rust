use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
    pub y: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "type")]
    pub image_type: i32,
    #[serde(default)]
    pub status: i32,
    #[serde(default)]
    pub resolution: Resolution,
}
