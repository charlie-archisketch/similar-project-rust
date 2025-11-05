use std::cmp::Ordering;

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
    pub area: f64,
    pub score: f64,
}

#[derive(Clone, Debug)]
pub struct FloorStructureRecord {
    pub id: String,
    pub title: String,
    pub project_id: String,
    pub area: f64,
    pub room_count: i32,
    pub bounding_box_width: f64,
    pub bounding_box_depth: f64,
    pub bounding_box_area: f64,
    pub bounding_box_aspect: f64,
    pub rectangularity: f64,
}

impl From<FloorStructureRecord> for floor_structure::ActiveModel {
    fn from(record: FloorStructureRecord) -> Self {
        floor_structure::ActiveModel {
            id: Set(record.id),
            title: Set(record.title),
            project_id: Set(record.project_id),
            area: Set(record.area),
            room_count: Set(record.room_count),
            bounding_box_width: Set(record.bounding_box_width),
            bounding_box_depth: Set(record.bounding_box_depth),
            bounding_box_area: Set(record.bounding_box_area),
            bounding_box_aspect: Set(record.bounding_box_aspect),
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
        room_count: i32,
        area_from: f64,
        area_to: f64,
        aspect: f64,
        rectangularity: f64,
        k: u64,
    ) -> Result<Vec<SimilarFloor>, ApiError> {
        if k == 0 {
            return Ok(Vec::new());
        }

        let area_dist = Func::abs(
            Expr::col((floor_structure::Entity, FloorStructureColumn::Area)).sub(Expr::value(area)),
        )
        .div(Expr::value(area.max(30.0_f64)));
        let aspect_dist = Func::abs(
            Expr::col((
                floor_structure::Entity,
                FloorStructureColumn::BoundingBoxAspect,
            ))
            .sub(Expr::value(aspect)),
        );
        let rectangularity_dist = Func::abs(
            Expr::col((
                floor_structure::Entity,
                FloorStructureColumn::Rectangularity,
            ))
            .sub(Expr::value(rectangularity)),
        );
        let room_count_diff = Func::abs(
            Expr::col((floor_structure::Entity, FloorStructureColumn::RoomCount))
                .sub(Expr::value(room_count as f64)),
        );
        let room_count_dist = room_count_diff
            .clone()
            .div(room_count_diff.add(Expr::value(room_count.max(1) as f64)));

        let score_expr = area_dist
            .clone()
            .mul(Expr::value(0.3_f64))
            .add(aspect_dist.clone().mul(Expr::value(0.3_f64)))
            .add(rectangularity_dist.clone().mul(Expr::value(0.2_f64)))
            .add(room_count_dist.clone().mul(Expr::value(0.2_f64)));

        let score_alias = Alias::new("score");
        let subquery_alias = Alias::new("distinct_floors");

        let mut distinct_per_project = Query::select();
        distinct_per_project
            .distinct_on([FloorStructureColumn::ProjectId])
            .column((floor_structure::Entity, FloorStructureColumn::Id))
            .column((floor_structure::Entity, FloorStructureColumn::Title))
            .column((floor_structure::Entity, FloorStructureColumn::ProjectId))
            .expr_as(
                Expr::col((floor_structure::Entity, FloorStructureColumn::Area)),
                Alias::new("area"),
            )
            .expr_as(score_expr.clone(), score_alias.clone())
            .from(floor_structure::Entity)
            .and_where(
                Expr::col((floor_structure::Entity, FloorStructureColumn::ProjectId))
                    .ne(exclude_project_id),
            )
            .and_where(
                Expr::col((floor_structure::Entity, FloorStructureColumn::Area))
                    .between(Expr::value(area_from), Expr::value(area_to)),
            )
            .and_where(
                Expr::col((
                    floor_structure::Entity,
                    FloorStructureColumn::BoundingBoxAspect,
                ))
                .between(
                    Expr::value(aspect * 0.85_f64),
                    Expr::value(aspect * 1.15_f64),
                ),
            )
            .and_where(
                Expr::col((
                    floor_structure::Entity,
                    FloorStructureColumn::Rectangularity,
                ))
                .between(
                    Expr::value(rectangularity - 0.1_f64),
                    Expr::value(rectangularity + 0.1_f64),
                ),
            )
            .and_where(
                Expr::col((floor_structure::Entity, FloorStructureColumn::RoomCount))
                    .between(Expr::value(room_count - 3), Expr::value(room_count + 3)),
            )
            .order_by(
                (floor_structure::Entity, FloorStructureColumn::ProjectId),
                Order::Asc,
            )
            .order_by_expr(score_expr.clone(), Order::Asc);

        let mut ordered_select = Query::select();
        ordered_select
            .column((subquery_alias.clone(), Alias::new("id")))
            .column((subquery_alias.clone(), Alias::new("title")))
            .column((subquery_alias.clone(), Alias::new("project_id")))
            .column((subquery_alias.clone(), Alias::new("area")))
            .column((subquery_alias.clone(), score_alias.clone()))
            .from_subquery(distinct_per_project, subquery_alias.clone())
            .order_by((subquery_alias.clone(), score_alias.clone()), Order::Asc)
            .order_by((subquery_alias.clone(), Alias::new("id")), Order::Asc)
            // .limit(k)
            ;

        let backend: DatabaseBackend = self.db.get_database_backend();
        let stmt: Statement = backend.build(&ordered_select);

        let mut results = SimilarFloor::find_by_statement(stmt)
            .all(&self.db)
            .await
            .map_err(ApiError::internal)?;

        results.sort_by(|a, b| match a.score.partial_cmp(&b.score) {
            Some(ordering) if ordering != Ordering::Equal => ordering,
            _ => a.id.cmp(&b.id),
        });

        Ok(results)
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
                            FloorStructureColumn::RoomCount,
                            FloorStructureColumn::BoundingBoxWidth,
                            FloorStructureColumn::BoundingBoxDepth,
                            FloorStructureColumn::BoundingBoxArea,
                            FloorStructureColumn::BoundingBoxAspect,
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
