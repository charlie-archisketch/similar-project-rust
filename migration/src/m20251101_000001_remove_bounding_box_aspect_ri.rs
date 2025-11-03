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
                    .drop_column(FloorStructures::BoundingBoxAspectRi)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(RoomStructures::Table)
                    .drop_column(RoomStructures::BoundingBoxAspectRi)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FloorStructures::Table)
                    .add_column(
                        ColumnDef::new(FloorStructures::BoundingBoxAspectRi)
                            .double()
                            .not_null()
                            .default(1.0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(RoomStructures::Table)
                    .add_column(
                        ColumnDef::new(RoomStructures::BoundingBoxAspectRi)
                            .double()
                            .not_null()
                            .default(1.0),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FloorStructures {
    Table,
    BoundingBoxAspectRi,
}

#[derive(DeriveIden)]
enum RoomStructures {
    Table,
    BoundingBoxAspectRi,
}
