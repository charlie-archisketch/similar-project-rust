pub use sea_orm_migration::prelude::*;

mod m20250224_000001_create_structures_tables;
mod m20251023_070251_create_project_table;
mod m20251028_074726_add_room_count_column_at_floor_structure;
mod m20251101_000001_remove_bounding_box_aspect_ri;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250224_000001_create_structures_tables::Migration),
            Box::new(m20251023_070251_create_project_table::Migration),
            Box::new(m20251028_074726_add_room_count_column_at_floor_structure::Migration),
            Box::new(m20251101_000001_remove_bounding_box_aspect_ri::Migration),
        ]
    }
}
