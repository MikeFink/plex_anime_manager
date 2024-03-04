use ::entity::{plex_event, plex_event::Entity as PlexEvent};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_event_by_id(db: &DbConn, id: i32) -> Result<Option<plex_event::Model>, DbErr> {
        PlexEvent::find_by_id(id).one(db).await
    }

    /// If ok, returns (post models, num pages).
    pub async fn find_events_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<plex_event::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = PlexEvent::find()
            .order_by_asc(plex_event::Column::Id)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
