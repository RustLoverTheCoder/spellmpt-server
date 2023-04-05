use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Prompt::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Prompt::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Prompt::Title).string().not_null())
                    .col(ColumnDef::new(Prompt::Content).string().not_null())
                    .col(ColumnDef::new(Prompt::Status).integer().not_null())
                    .col(ColumnDef::new(Prompt::Type).integer().not_null())
                    .col(ColumnDef::new(Prompt::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Prompt::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Prompt::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Prompt::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Prompt {
    Table,
    Id,
    Title,
    Content,
    Status,
    Type,
    UserId,
    CreatedAt,
    UpdatedAt,
}
