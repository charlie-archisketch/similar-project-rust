use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FloorStructures::Table)
                    .rename_column(
                        FloorStructures::BoundingBoxHeight,
                        FloorStructures::BoundingBoxDepth,
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(RoomStructures::Table)
                    .rename_column(
                        RoomStructures::BoundingBoxHeight,
                        RoomStructures::BoundingBoxDepth,
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FloorStructures::Table)
                    .rename_column(
                        FloorStructures::BoundingBoxDepth,
                        FloorStructures::BoundingBoxHeight,
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(RoomStructures::Table)
                    .rename_column(
                        RoomStructures::BoundingBoxDepth,
                        RoomStructures::BoundingBoxHeight,
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FloorStructures {
    #[sea_orm(iden = "floor_structures")]
    Table,
    #[sea_orm(iden = "bounding_box_height")]
    BoundingBoxHeight,
    #[sea_orm(iden = "bounding_box_depth")]
    BoundingBoxDepth,
}

#[derive(DeriveIden)]
enum RoomStructures {
    #[sea_orm(iden = "room_structures")]
    Table,
    #[sea_orm(iden = "bounding_box_height")]
    BoundingBoxHeight,
    #[sea_orm(iden = "bounding_box_depth")]
    BoundingBoxDepth,
}
