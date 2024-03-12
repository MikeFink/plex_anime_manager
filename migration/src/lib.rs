pub use sea_orm_migration::prelude::*;

mod m20240305_141836_create_anime;
mod m20240305_142140_create_watched_episodes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240305_141836_create_anime::Migration),
            Box::new(m20240305_142140_create_watched_episodes::Migration),
        ]
    }
}
