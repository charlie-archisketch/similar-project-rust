use sea_orm::{
    ActiveValue::Set,
    ConnectionTrait, DatabaseBackend, DatabaseConnection, EntityTrait, FromQueryResult, Order,
    Statement,
    sea_query::{Alias, Expr, ExprTrait, Func, OnConflict, Query},
};

use crate::{
    error::ApiError,
    models::project::structure::{FloorStructureColumn, FloorStructureEntity, floor_structure},
};

#[derive(Clone)]
pub struct FloorStructureRepository {
    db: DatabaseConnection,
}

#[allow(dead_code)]
#[derive(Debug, Clone, FromQueryResult)]
pub struct SimilarFloor {
    pub id: String,
    pub title: String,
    pub project_id: String,
    pub score: f64,
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

    pub async fn find_by_id(&self, id: &str) -> Result<Option<floor_structure::Model>, ApiError> {
        FloorStructureEntity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(ApiError::internal)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn find_top_k_similar_floors(
        &self,
        exclude_project_id: &str,
        area: f64,
        aspect_ri: f64,
        rectangularity: f64,
        k: u64,
    ) -> Result<Vec<SimilarFloor>, ApiError> {
        if k == 0 {
            return Ok(Vec::new());
        }

        let area_denominator = if area.abs() < f64::EPSILON { 1.0 } else { area };

        let area_dist = Func::abs(
            Expr::col((floor_structure::Entity, FloorStructureColumn::Area)).sub(Expr::value(area)),
        )
        .div(Expr::value(area_denominator));
        let aspect_dist = Func::abs(
            Expr::col((
                floor_structure::Entity,
                FloorStructureColumn::BoundingBoxAspectRi,
            ))
            .sub(Expr::value(aspect_ri)),
        )
        .div(Expr::value(0.3_f64));
        let rectangularity_dist = Func::abs(
            Expr::col((
                floor_structure::Entity,
                FloorStructureColumn::Rectangularity,
            ))
            .sub(Expr::value(rectangularity)),
        )
        .div(Expr::value(0.1_f64));

        let score_expr = area_dist
            .clone()
            .mul(Expr::value(0.3_f64))
            .add(aspect_dist.clone().mul(Expr::value(0.5_f64)))
            .add(rectangularity_dist.clone().mul(Expr::value(0.2_f64)));

        let score_alias = Alias::new("score");

        let mut select = Query::select();
        select
            .column((floor_structure::Entity, FloorStructureColumn::Id))
            .column((floor_structure::Entity, FloorStructureColumn::Title))
            .column((floor_structure::Entity, FloorStructureColumn::ProjectId))
            .expr_as(score_expr.clone(), score_alias.clone())
            .from(floor_structure::Entity)
            .and_where(
                Expr::col((floor_structure::Entity, FloorStructureColumn::ProjectId))
                    .ne(exclude_project_id),
            )
            .order_by(score_alias.clone(), Order::Asc)
            .limit(k);

        let backend: DatabaseBackend = self.db.get_database_backend();
        let stmt: Statement = backend.build(&select);

        SimilarFloor::find_by_statement(stmt)
            .all(&self.db)
            .await
            .map_err(ApiError::internal)
    }

    pub async fn save_all(&self, records: Vec<FloorStructureRecord>) -> Result<(), ApiError> {
        if records.is_empty() {
            return Ok(());
        }

        for record in records {
            let model: floor_structure::ActiveModel = record.into();
            FloorStructureEntity::insert(model)
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
        }

        Ok(())
    }
}
