pub use sea_orm_migration::prelude::*;

mod m20231017_024420_create_manga_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231017_024420_create_manga_table::Migration),
        ]
    }
}
