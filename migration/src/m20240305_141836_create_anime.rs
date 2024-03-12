use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Anime::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Anime::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Anime::Title)
                        .string()
                        .not_null())
                    .col(ColumnDef::new(Anime::AgentId)
                        .integer()
                        .not_null())
                    .to_owned()
                    .col(ColumnDef::new(Anime::ExternalId)
                        .integer()
                        .not_null())
                        .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Anime::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Anime {
    Table,
    Id,
    Title,
    AgentId,
    ExternalId
}
