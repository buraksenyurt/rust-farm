mod model;
mod repository;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Database;

    #[tokio::test]
    async fn should_connecting_db_works_test() {
        let db = Database::connect().await;
        assert!(db.is_ok())
    }
}
