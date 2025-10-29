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
                    .add_column(
                        ColumnDef::new(FloorStructures::RoomCount)
                            .integer()
                            .not_null()
                            .default(1),
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
                    .drop_column(FloorStructures::RoomCount)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FloorStructures {
    Table,
    RoomCount,
}
