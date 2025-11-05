use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .rename_table(
                Table::rename()
                    .table(FloorStructures::Table, Floors::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .rename_table(
                Table::rename()
                    .table(RoomStructures::Table, Rooms::Table)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .rename_table(
                Table::rename()
                    .table(Floors::Table, FloorStructures::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .rename_table(
                Table::rename()
                    .table(Rooms::Table, RoomStructures::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FloorStructures {
    #[sea_orm(iden = "floor_structures")]
    Table,
}

#[derive(DeriveIden)]
enum Floors {
    #[sea_orm(iden = "floors")]
    Table,
}

#[derive(DeriveIden)]
enum RoomStructures {
    #[sea_orm(iden = "room_structures")]
    Table,
}

#[derive(DeriveIden)]
enum Rooms {
    #[sea_orm(iden = "rooms")]
    Table,
}
