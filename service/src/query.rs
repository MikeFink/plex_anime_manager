use ::entity::{watched_episodes, watched_episodes::Entity as WatchedEpisode};
use ::entity::{anime, anime::Entity as Anime};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_anime_by_id(db: &DbConn, id: i32) -> Result<Option<anime::Model>, DbErr> {
        return Anime::find_by_id(id)
            .one(db)
            .await;
    }

    pub async fn find_anime_by_agent_id(db: &DbConn, agent_id: i32, external_id: i32) -> Result<Option<anime::Model>, DbErr> {
        return Anime::find()
            .filter(anime::Column::AgentId.eq(agent_id))
            .filter(anime::Column::ExternalId.eq(external_id))
            .one(db)
            .await;
    }

    pub async fn find_anime_with_watched_episodes_by_id(db: &DbConn, id: i32) -> Result<Vec<(anime::Model, Option<watched_episodes::Model>)>, DbErr> {
        return Anime::find_by_id(id)
            .find_also_related(WatchedEpisode)
            .all(db)
            .await;
    }

    pub async fn find_anime_in_page(
        db: &DbConn,
        page: u64,
        anime_per_page: u64,
    ) -> Result<(Vec<anime::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Anime::find()
            .order_by_asc(anime::Column::Id)
            .paginate(db, anime_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated posts
        return paginator.fetch_page(page - 1).await.map(|p| (p, num_pages));
    }
}
