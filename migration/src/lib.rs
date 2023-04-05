pub use sea_orm_migration::prelude::*;

mod m20230310_032859_create_user_table;
mod m20230405_111835_create_prompt_table;
mod m20230405_142124_create_tag_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230310_032859_create_user_table::Migration),
            Box::new(m20230405_111835_create_prompt_table::Migration),
            Box::new(m20230405_142124_create_tag_table::Migration),
        ]
    }
}
