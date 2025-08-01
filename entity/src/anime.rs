//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "anime")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub agent_id: i32,
    pub external_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::watched_episodes::Entity")]
    WatchedEpisodes,
}

impl Related<super::watched_episodes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WatchedEpisodes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
