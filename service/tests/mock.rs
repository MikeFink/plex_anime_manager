mod prepare;

use service::{Mutation, Query};
use entity::plex_event;
use prepare::prepare_mock_db;

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db();

    {
        let post = Query::find_event_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(post.id, 1);
    }

    {
        let post = Query::find_event_by_id(db, 5).await.unwrap().unwrap();

        assert_eq!(post.id, 5);
    }

    {
        let plex_event = Mutation::create_event(
            db,
            plex_event::Model {
                id: 0,
                title: "Title D".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            plex_event,
            plex_event::ActiveModel {
                id: sea_orm::ActiveValue::Unchanged(6),
                title: sea_orm::ActiveValue::Unchanged("Title D".to_owned()),
            }
        );
    }

    {
        let plex_event = Mutation::update_event_by_id(
            db,
            1,
            plex_event::Model {
                id: 1,
                title: "New Title A".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            plex_event,
            plex_event::Model {
                id: 1,
                title: "New Title A".to_owned(),
            }
        );
    }

    {
        let result = Mutation::delete_event(db, 5).await.unwrap();

        assert_eq!(result.rows_affected, 1);
    }

    {
        let result = Mutation::delete_all_events(db).await.unwrap();

        assert_eq!(result.rows_affected, 5);
    }
}
