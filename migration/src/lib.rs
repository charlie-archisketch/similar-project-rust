pub use sea_orm_migration::prelude::*;

mod m20250224_000001_create_structures_tables;
mod m20251023_070251_create_project_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250224_000001_create_structures_tables::Migration),
            Box::new(m20251023_070251_create_project_table::Migration),
        ]
    }
}
