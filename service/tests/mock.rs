mod prepare;

#[cfg(test)]
mod test {

    use crate::prepare::prepare_mock_db;
    use service::GuestService;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_mock_find_by_id() {
        let db = &prepare_mock_db();

        let uuid = Uuid::parse_str("b04965e6-a9bb-591f-8f8a-1adcb2c8dc39");
        let id = uuid.unwrap();
        let result = GuestService::find_by_id(db, id).await.unwrap().unwrap();

        assert_eq!(result.id, id);
    }

    #[tokio::test]
    async fn test_mock_delete() {
        let db = &prepare_mock_db();
        let uuid = Uuid::parse_str("b04965e6-a9bb-591f-8f8a-1adcb2c8dc39");
        let id = uuid.unwrap();
        let result = GuestService::delete(db, id).await.unwrap();
        assert_eq!(result.rows_affected, 1);

        let result = GuestService::delete_all(db).await.unwrap();
        assert_eq!(result.rows_affected, 3);
    }
}
