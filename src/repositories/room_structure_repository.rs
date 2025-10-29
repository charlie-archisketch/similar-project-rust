use sea_orm::{
    ActiveValue::Set,
    ConnectionTrait, DatabaseBackend, DatabaseConnection, EntityTrait, FromQueryResult, Order,
    Statement,
    sea_query::{Alias, Expr, ExprTrait, Func, OnConflict, Query},
};

use crate::{
    error::ApiError,
    models::project::structure::{RoomStructureColumn, RoomStructureEntity, room_structure},
};

#[derive(Clone)]
pub struct RoomStructureRepository {
    db: DatabaseConnection,
}

#[allow(dead_code)]
#[derive(Debug, Clone, FromQueryResult)]
pub struct SimilarRoom {
    pub id: String,
    pub project_id: String,
    pub score: f64,
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

    pub async fn find_by_id(&self, id: &str) -> Result<Option<room_structure::Model>, ApiError> {
        RoomStructureEntity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map_err(ApiError::internal)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn find_top_k_similar_rooms(
        &self,
        exclude_project_id: &str,
        area: f64,
        area_from: f64,
        area_to: f64,
        rectangularity: f64,
        aspect_ri: f64,
        room_type: i32,
        k: u64,
    ) -> Result<Vec<SimilarRoom>, ApiError> {
        if k == 0 {
            return Ok(Vec::new());
        }

        let area_dist = Func::abs(
            Expr::col((room_structure::Entity, RoomStructureColumn::Area)).sub(Expr::value(area)),
        )
        .div(Expr::value(area.max(5.0_f64)));
        let aspect_dist = Func::abs(
            Expr::col((
                room_structure::Entity,
                RoomStructureColumn::BoundingBoxAspectRi,
            ))
            .sub(Expr::value(aspect_ri)),
        );
        let rectangularity_dist = Func::abs(
            Expr::col((room_structure::Entity, RoomStructureColumn::Rectangularity))
                .sub(Expr::value(rectangularity)),
        );

        let score_expr = area_dist
            .clone()
            .mul(Expr::value(0.3_f64))
            .add(aspect_dist.clone().mul(Expr::value(0.5_f64)))
            .add(rectangularity_dist.clone().mul(Expr::value(0.2_f64)));

        let score_alias = Alias::new("score");

        let mut select = Query::select();
        select
            .column((room_structure::Entity, RoomStructureColumn::Id))
            .column((room_structure::Entity, RoomStructureColumn::ProjectId))
            .expr_as(score_expr.clone(), score_alias.clone())
            .from(room_structure::Entity)
            .and_where(
                Expr::col((room_structure::Entity, RoomStructureColumn::ProjectId))
                    .ne(exclude_project_id),
            )
            .and_where(
                Expr::col((room_structure::Entity, RoomStructureColumn::Area))
                    .between(Expr::value(area_from), Expr::value(area_to)),
            )
            .and_where(
                Expr::col((
                    room_structure::Entity,
                    RoomStructureColumn::BoundingBoxAspectRi,
                ))
                .between(
                    Expr::value(aspect_ri * 0.85_f64),
                    Expr::value(aspect_ri * 1.15_f64),
                ),
            )
            .and_where(
                Expr::col((room_structure::Entity, RoomStructureColumn::Rectangularity)).between(
                    Expr::value(rectangularity - 0.1_f64),
                    Expr::value(rectangularity + 0.1_f64),
                ),
            );

        if room_type != 0 {
            select.and_where(
                Expr::col((room_structure::Entity, RoomStructureColumn::Type)).eq(room_type),
            );
        }

        select.order_by(score_alias.clone(), Order::Asc).limit(k);

        let backend: DatabaseBackend = self.db.get_database_backend();
        let stmt: Statement = backend.build(&select);

        SimilarRoom::find_by_statement(stmt)
            .all(&self.db)
            .await
            .map_err(ApiError::internal)
    }

    pub async fn save_all(&self, records: Vec<RoomStructureRecord>) -> Result<(), ApiError> {
        if records.is_empty() {
            return Ok(());
        }

        for record in records {
            let model: room_structure::ActiveModel = record.into();
            RoomStructureEntity::insert(model)
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
        }

        Ok(())
    }
}
