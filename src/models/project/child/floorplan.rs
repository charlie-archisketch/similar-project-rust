use std::collections::HashMap;

use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::models::common::Transformation;
use crate::models::material::{Color, Texture, Vector2};
use crate::models::project::enums::{ColumnType, FinishTargetType};

use crate::models::project::child::portfolio::Price;

fn generate_object_id() -> String {
    ObjectId::new().to_string()
}

fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Floorplan {
    #[serde(default = "generate_object_id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archi_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floorplan_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floorplan_image_scale: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fp_image_scale: Option<FpImageScale>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corners: Option<Vec<Corner>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub walls: Option<Vec<Wall>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<Point>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<Line>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rooms: Option<Vec<Room>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Item>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finish_items: Option<Vec<FinishItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups_v2: Option<Vec<GroupV2>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub light: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub light_sources: Option<Vec<LightSource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dxf_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl Floorplan {
    pub fn new() -> Self {
        Self {
            id: generate_object_id(),
            archi_id: None,
            title: None,
            floorplan_image: None,
            floorplan_image_scale: None,
            fp_image_scale: None,
            area: None,
            dimensions: None,
            corners: Some(Vec::new()),
            walls: Some(Vec::new()),
            points: Some(Vec::new()),
            lines: Some(Vec::new()),
            rooms: Some(Vec::new()),
            items: Some(Vec::new()),
            finish_items: Some(Vec::new()),
            groups: Some(Vec::new()),
            columns: Some(Vec::new()),
            groups_v2: Some(Vec::new()),
            light: None,
            light_sources: Some(Vec::new()),
            dxf_url: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn mark_created(&mut self) {
        let now = Utc::now();
        self.created_at = Some(now);
        self.updated_at = Some(now);
    }

    pub fn touch(&mut self) {
        self.updated_at = Some(Utc::now());
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FpImageScale {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<Transformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<Transformation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Corner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archi_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Transformation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wall {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archi_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corners: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(
        default = "wall_default_level",
        skip_serializing_if = "Option::is_none"
    )]
    pub level: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thickness: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finishes: Option<Vec<Finish>>,
}

fn wall_default_level() -> Option<f64> {
    Some(0.0)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archi_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Position2D>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archi_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position2D {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    #[serde(default = "generate_uuid")]
    pub archi_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(default)]
    pub corners: Vec<String>,
    #[serde(default = "room_default_height")]
    pub height: f64,
    #[serde(
        default = "room_default_level",
        skip_serializing_if = "Option::is_none"
    )]
    pub level: Option<f64>,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub r#type: i32,
    #[serde(default)]
    pub hide_ceiling: bool,
    #[serde(default)]
    pub finish: Finish,
    #[serde(default)]
    pub ceiling: Finish,
    #[serde(default)]
    pub inner_points: Vec<Transformation>,
    #[serde(default)]
    pub lock: bool,
    #[serde(
        default = "room_default_visible",
        skip_serializing_if = "Option::is_none"
    )]
    pub visible: Option<bool>,
    #[serde(default)]
    pub items: Vec<Item>,
    #[serde(default)]
    pub seats: i32,
    #[serde(
        default,
        deserialize_with = "crate::utils::serde::deserialize_f64_or_default"
    )]
    pub area: f64,
}

fn room_default_height() -> f64 {
    1300.0
}

fn room_default_level() -> Option<f64> {
    Some(0.0)
}

fn room_default_visible() -> Option<bool> {
    Some(true)
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Finish {
    #[serde(default = "generate_object_id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(
        default = "default_finish_color",
        skip_serializing_if = "Option::is_none"
    )]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<Value>,
    #[serde(default)]
    pub offset: Offset2D,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<Surface>,
}

fn default_finish_color() -> Option<String> {
    Some("ffffff".to_string())
}

impl Default for Finish {
    fn default() -> Self {
        Self {
            id: generate_object_id(),
            product_id: None,
            color: default_finish_color(),
            meta: None,
            scheme: None,
            offset: Offset2D::default(),
            rotation: 0.0,
            surface: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offset2D {
    pub x: f64,
    pub y: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<Texture>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_image: Option<Texture>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_image: Option<Texture>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_scale: Option<Vector2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roughness_image: Option<Texture>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roughness: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metal_image: Option<Texture>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metalness: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transparent: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opacity_image: Option<Texture>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub displacement_image: Option<Texture>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub displacement_scale: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emissive_image: Option<Texture>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emissive_intensity: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialSnapshot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<Material>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archi_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parametric_info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Transformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<Transformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<Transformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pivot: Option<Transformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_transform: Option<Vec<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ItemComponent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actived_materials: Option<Vec<MaterialSnapshot>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemComponent {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "enterpriseCode")]
    pub enterprise_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<ComponentRelation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component: Option<Component>,
    #[serde(default)]
    pub child_components: Vec<ItemComponent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComponentGroupRule {
    MinMax,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentGroupInfo {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub rule: ComponentGroupRule,
    pub rule_min: i32,
    pub rule_max: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentRelation {
    pub quantity: f64,
    #[serde(rename = "isRequired")]
    pub is_required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_info: Option<ComponentGroupInfo>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_material: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_material: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<Dimension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retail_price: Option<Price>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archi_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inbound_box: Option<Dimension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<Vec<Member>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation_products: Option<RelationProduct>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation_products_v2: Option<RelationProductV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Vec<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decriminator: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupV2 {
    pub archi_id: String,
    pub position: Transformation,
    pub rotation: Transformation,
    pub scale: Transformation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    pub lock: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    pub children: Vec<GroupChild>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation_products_v2: Option<RelationProductV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<HashMap<String, Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assembly_group_info: Option<AssemblyGroupInfo>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChildType {
    Item,
    Group,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupChild {
    pub archi_id: String,
    pub r#type: ChildType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssemblyGroupInfo {
    pub select_root_category_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archi_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationProduct {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationProductV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_panel: Option<EndPanel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndPanel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archi_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub archi_id: String,
    pub position: Transformation,
    pub r#type: ColumnType,
    pub rotation: Transformation,
    pub scale: Transformation,
    pub lock: bool,
    pub visible: bool,
    pub planes: Vec<Plane>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plane {
    pub archi_id: String,
    pub finish: Finish,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Euler {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LightBase {
    #[serde(default = "generate_object_id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archi_id: Option<String>,
    #[serde(rename = "type")]
    pub light_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Transformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_temp: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_rgb: Option<Rgb>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brightness: Option<i32>,
}

impl LightBase {
    pub fn new(light_type: &str) -> Self {
        Self {
            id: generate_object_id(),
            archi_id: None,
            light_type: light_type.to_string(),
            status: None,
            position: None,
            color_type: None,
            color_temp: None,
            color_rgb: None,
            brightness: None,
        }
    }

    pub fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(other) = other {
            if other.archi_id.is_some() {
                self.archi_id = other.archi_id.clone();
            }
            if other.status.is_some() {
                self.status = other.status;
            }
            if other.position.is_some() {
                self.position = other.position.clone();
            }
            if other.color_type.is_some() {
                self.color_type = other.color_type.clone();
            }
            if other.color_temp.is_some() {
                self.color_temp = other.color_temp;
            }
            if other.color_rgb.is_some() {
                self.color_rgb = other.color_rgb.clone();
            }
            if other.brightness.is_some() {
                self.brightness = other.brightness;
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaLight {
    #[serde(flatten)]
    pub base: LightBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<Euler>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portal: Option<bool>,
}

impl Default for AreaLight {
    fn default() -> Self {
        Self {
            base: LightBase::new("area"),
            width: None,
            height: None,
            direction: None,
            portal: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointLight {
    #[serde(flatten)]
    pub base: LightBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            base: LightBase::new("point"),
            radius: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spotlight {
    #[serde(flatten)]
    pub base: LightBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ies_texture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<Euler>,
}

impl Default for Spotlight {
    fn default() -> Self {
        Self {
            base: LightBase::new("spot"),
            ies_texture: None,
            direction: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LightSource {
    #[serde(default = "generate_object_id")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Value>>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floor_archi_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_archi_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wall_archi_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    pub price: i32,
    pub quantity: i32,
    pub loss: f64,
    pub target_area: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<FinishTargetType>,
    #[serde(default)]
    pub tile_quantity: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Surface {
    pub archi_id: String,
    #[serde(default)]
    pub points: HashMap<String, SurfacePoint>,
    #[serde(default)]
    pub lines: HashMap<String, SurfaceLine>,
    #[serde(default)]
    pub areas: HashMap<String, SurfaceArea>,
    #[serde(default)]
    pub guidelines: HashMap<String, SurfaceGuideline>,
    pub thickness: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfacePoint {
    pub archi_id: String,
    pub r#type: String,
    pub coordinates: Position2D,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceLine {
    pub archi_id: String,
    pub r#type: String,
    pub start: String,
    pub end: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arc: Option<SurfaceArc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SurfaceLineProperties>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceArc {
    pub center: Position2D,
    pub radius: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceLineProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub light_strip: Option<SurfaceLightStrip>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub light_trough: Option<SurfaceLightTrough>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moldings: Option<Vec<SurfaceMolding>>,
    #[serde(
        rename = "isInitialShape",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_initial_shape: Option<bool>,
    #[serde(
        rename = "isInitialHole",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_initial_hole: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceLightStrip {
    pub archi_id: String,
    pub r#type: String,
    pub brightness: f64,
    pub color: String,
    pub radius: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceLightTrough {
    pub archi_id: String,
    pub path: SurfacePolygon,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceDeserializedLine {
    pub archi_id: String,
    pub r#type: String,
    pub start: SurfacePoint,
    pub end: SurfacePoint,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arc: Option<SurfaceArc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfacePolygon {
    pub archi_id: String,
    pub r#type: String,
    pub shape: Vec<SurfaceDeserializedLine>,
    #[serde(default)]
    pub holes: Vec<Vec<SurfaceDeserializedLine>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceMolding {
    pub archi_id: String,
    pub placement: String,
    pub product_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    pub path_transformation: SurfaceTransformation,
    pub finishes: Vec<SurfaceFinish>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceTransformation {
    pub offset: Position2D,
    pub rotation: f64,
    pub scale: Position2D,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SurfaceFinish {
    Color(SurfaceColorFinish),
    Product(SurfaceProductFinish),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceColorFinish {
    pub archi_id: String,
    pub color: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceProductFinish {
    pub archi_id: String,
    pub product_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    pub transformation: SurfaceTransformation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceArea {
    pub archi_id: String,
    pub r#type: String,
    pub shape: Vec<String>,
    #[serde(default)]
    pub holes: Vec<Vec<String>>,
    pub properties: SurfaceAreaProperties,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceAreaProperties {
    pub finish: SurfaceFinish,
    pub extrusion: SurfaceAreaExtrusion,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceAreaExtrusion {
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceGuideline {
    pub archi_id: String,
    pub r#type: String,
    pub start: SurfacePoint,
    pub end: SurfacePoint,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ItemExtension {
    #[serde(rename = "Kitchen")]
    Kitchen,
    #[serde(rename = "Bath")]
    Bath,
    #[serde(rename = "Part")]
    Part,
}
