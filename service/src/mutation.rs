use ::entity::{anime, watched_episodes};
use sea_orm::*;
use crate::Query;

pub struct Mutation;

impl Mutation {
    pub async fn find_or_create_anime(
        db: &DbConn,
        title: String,
        agent_id: i32,
        external_id: i32
    ) -> Result<anime::Model, DbErr> {
        let existing_anime = Query::find_anime_by_agent_id(db, agent_id, external_id).await?;

        match existing_anime {
            Some(existing_anime) => Ok(existing_anime),
            None => {
                let new_anime = anime::ActiveModel {
                    title: Set(title),
                    agent_id: Set(agent_id.to_owned()),
                    external_id: Set(external_id.to_owned()),
                    ..Default::default()
                }
                .save(db)
                .await?;

                let new_anime = new_anime.try_into_model()?;
                Ok(new_anime)
            }
        }
    }

    pub async fn create_event(
        db: &DbConn,
        index: i32,
        parent_index: i32,
        anime_id: i32
    ) -> Result<watched_episodes::ActiveModel, DbErr> {
        watched_episodes::ActiveModel {
            index: Set(index.to_owned()),
            parent_index: Set(parent_index.to_owned()),
            anime_id: Set(anime_id.to_owned()),
            ..Default::default()
        }
            .save(db)
            .await
    }
}
