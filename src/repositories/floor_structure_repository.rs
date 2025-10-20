use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait, sea_query::OnConflict};

use crate::{
    error::ApiError,
    models::project::structure::{FloorStructureColumn, FloorStructureEntity, floor_structure},
};

#[derive(Clone)]
pub struct FloorStructureRepository {
    db: DatabaseConnection,
}

#[derive(Clone, Debug)]
pub struct FloorStructureRecord {
    pub id: String,
    pub title: String,
    pub project_id: String,
    pub area: f64,
    pub bounding_box_width: f64,
    pub bounding_box_height: f64,
    pub bounding_box_area: f64,
    pub bounding_box_aspect: f64,
    pub bounding_box_aspect_ri: f64,
    pub rectangularity: f64,
}

impl From<FloorStructureRecord> for floor_structure::ActiveModel {
    fn from(record: FloorStructureRecord) -> Self {
        floor_structure::ActiveModel {
            id: Set(record.id),
            title: Set(record.title),
            project_id: Set(record.project_id),
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

impl FloorStructureRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn save_all(&self, records: Vec<FloorStructureRecord>) -> Result<(), ApiError> {
        if records.is_empty() {
            return Ok(());
        }

        let models: Vec<floor_structure::ActiveModel> =
            records.into_iter().map(Into::into).collect();

        FloorStructureEntity::insert_many(models)
            .on_conflict(
                OnConflict::column(FloorStructureColumn::Id)
                    .update_columns([
                        FloorStructureColumn::Title,
                        FloorStructureColumn::ProjectId,
                        FloorStructureColumn::Area,
                        FloorStructureColumn::BoundingBoxWidth,
                        FloorStructureColumn::BoundingBoxHeight,
                        FloorStructureColumn::BoundingBoxArea,
                        FloorStructureColumn::BoundingBoxAspect,
                        FloorStructureColumn::BoundingBoxAspectRi,
                        FloorStructureColumn::Rectangularity,
                    ])
                    .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(ApiError::internal)?;

        Ok(())
    }
}
