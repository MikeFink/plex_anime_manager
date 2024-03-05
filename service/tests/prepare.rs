use ::entity::plex_event;
use sea_orm::*;

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            [plex_event::Model {
                id: 1,
                title: "Title A".to_owned(),
            }],
            [plex_event::Model {
                id: 5,
                title: "Title C".to_owned(),
            }],
            [plex_event::Model {
                id: 6,
                title: "Title D".to_owned(),
            }],
            [plex_event::Model {
                id: 1,
                title: "Title A".to_owned(),
            }],
            [plex_event::Model {
                id: 1,
                title: "New Title A".to_owned(),
            }],
            [plex_event::Model {
                id: 5,
                title: "Title C".to_owned(),
            }],
        ])
        .append_exec_results([
            MockExecResult {
                last_insert_id: 6,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 6,
                rows_affected: 5,
            },
        ])
        .into_connection()
}
