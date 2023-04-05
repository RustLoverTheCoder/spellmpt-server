use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Taggable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Taggable::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Taggable::TagId).integer().not_null())
                    .col(ColumnDef::new(Taggable::TaggableId).uuid().not_null())
                    .col(ColumnDef::new(Taggable::TaggableType).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Taggable::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Taggable {
    Table,
    Id,
    TagId,
    TaggableId,
    TaggableType,
}
