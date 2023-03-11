pub use sea_orm_migration::prelude::*;

mod m20230310_032859_create_user_table;
mod m20230311_104539_create_block;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230310_032859_create_user_table::Migration),
            Box::new(m20230311_104539_create_block::Migration),
        ]
    }
}
