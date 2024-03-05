use ::entity::{plex_event, plex_event::Entity as PlexEvent};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_event(
        db: &DbConn,
        form_data: plex_event::Model,
    ) -> Result<plex_event::ActiveModel, DbErr> {
        plex_event::ActiveModel {
            title: Set(form_data.title.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_event_by_id(
        db: &DbConn,
        id: i32,
        form_data: plex_event::Model,
    ) -> Result<plex_event::Model, DbErr> {
        let post: plex_event::ActiveModel = PlexEvent::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        plex_event::ActiveModel {
            id: post.id,
            title: Set(form_data.title.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_event(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let post: plex_event::ActiveModel = PlexEvent::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn delete_all_events(db: &DbConn) -> Result<DeleteResult, DbErr> {
        PlexEvent::delete_many().exec(db).await
    }
}
