use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "rooms")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub project_id: String,
    pub r#type: i32,
    pub area: f64,
    pub bounding_box_width: f64,
    pub bounding_box_depth: f64,
    pub bounding_box_area: f64,
    pub bounding_box_aspect: f64,
    pub rectangularity: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
