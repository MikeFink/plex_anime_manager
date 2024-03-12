mod prepare;
use service::Query;
use prepare::prepare_mock_db;

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db();

    {
        let post = Query::find_anime_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(post.id, 1);
    }

    {
        let post = Query::find_anime_by_id(db, 5).await.unwrap().unwrap();

        assert_eq!(post.id, 5);
    }
}
