use ::entity::anime;
use sea_orm::*;

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            [anime::Model {
                id: 1,
                title: "Title A".to_owned(),
                agent_id: 0,
                external_id: 0,
            }],
            [anime::Model {
                id: 5,
                title: "Title C".to_owned(),
                agent_id: 0,
                external_id: 0,
            }],
            [anime::Model {
                id: 6,
                title: "Title D".to_owned(),
                agent_id: 0,
                external_id: 0,
            }],
            [anime::Model {
                id: 1,
                title: "Title A".to_owned(),
                agent_id: 0,
                external_id: 0,
            }],
            [anime::Model {
                id: 1,
                title: "New Title A".to_owned(),
                agent_id: 0,
                external_id: 0,
            }],
            [anime::Model {
                id: 5,
                title: "Title C".to_owned(),
                agent_id: 0,
                external_id: 0,
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
