use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait, sea_query::OnConflict};

use crate::{
    error::ApiError,
    models::project::structure::{RoomStructureColumn, RoomStructureEntity, room_structure},
};

#[derive(Clone)]
pub struct RoomStructureRepository {
    db: DatabaseConnection,
}

#[derive(Clone, Debug)]
pub struct RoomStructureRecord {
    pub id: String,
    pub project_id: String,
    pub r#type: i32,
    pub area: f64,
    pub bounding_box_width: f64,
    pub bounding_box_height: f64,
    pub bounding_box_area: f64,
    pub bounding_box_aspect: f64,
    pub bounding_box_aspect_ri: f64,
    pub rectangularity: f64,
}

impl From<RoomStructureRecord> for room_structure::ActiveModel {
    fn from(record: RoomStructureRecord) -> Self {
        room_structure::ActiveModel {
            id: Set(record.id),
            project_id: Set(record.project_id),
            r#type: Set(record.r#type),
            area: Set(record.area),
            bounding_box_width: Set(record.bounding_box_width),
            bounding_box_height: Set(record.bounding_box_height),
            bounding_box_area: Set(record.bounding_box_area),
            bounding_box_aspect: Set(record.bounding_box_aspect),
            bounding_box_aspect_ri: Set(record.bounding_box_aspect_ri),
            rectangularity: Set(record.rectangularity),
        }
    }
}

impl RoomStructureRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn save_all(&self, records: Vec<RoomStructureRecord>) -> Result<(), ApiError> {
        if records.is_empty() {
            return Ok(());
        }

        let models: Vec<room_structure::ActiveModel> =
            records.into_iter().map(Into::into).collect();

        RoomStructureEntity::insert_many(models)
            .on_conflict(
                OnConflict::column(RoomStructureColumn::Id)
                    .update_columns([
                        RoomStructureColumn::ProjectId,
                        RoomStructureColumn::Type,
                        RoomStructureColumn::Area,
                        RoomStructureColumn::BoundingBoxWidth,
                        RoomStructureColumn::BoundingBoxHeight,
                        RoomStructureColumn::BoundingBoxArea,
                        RoomStructureColumn::BoundingBoxAspect,
                        RoomStructureColumn::BoundingBoxAspectRi,
                        RoomStructureColumn::Rectangularity,
                    ])
                    .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(ApiError::internal)?;

        Ok(())
    }
}
