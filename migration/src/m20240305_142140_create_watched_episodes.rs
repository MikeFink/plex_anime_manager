use sea_orm_migration::prelude::*;
use crate::m20240305_141836_create_anime::Anime;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WatchedEpisodes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WatchedEpisodes::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    ).col(ColumnDef::new(WatchedEpisodes::Index)
                        .integer()
                        .not_null())
                    .col(ColumnDef::new(WatchedEpisodes::ParentIndex)
                        .integer()
                        .not_null())
                    .col(ColumnDef::new(WatchedEpisodes::AnimeId)
                        .integer()
                        .not_null())
                    .foreign_key(ForeignKey::create()
                        .name("fk-watchedEpisodes-anime-id")
                        .from(WatchedEpisodes::Table,WatchedEpisodes::AnimeId)
                        .to(Anime::Table, Anime::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WatchedEpisodes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum WatchedEpisodes {
    Table,
    Id,
    Index,
    ParentIndex,
    AnimeId
}
