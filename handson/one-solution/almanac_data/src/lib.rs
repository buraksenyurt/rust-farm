use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;
use tracing::log;

pub async fn get_conn(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(db_url.to_owned());
    options
        .connect_timeout(Duration::from_secs(10))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    return Database::connect(options).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
    use testcontainers::{clients, images};

    #[tokio::test]
    async fn should_db_connection_works_test() {
        // docker cli'a erişiyoruz
        let docker = clients::Cli::default();
        // standart Postgres imagını alıyoruz
        let db = images::postgres::Postgres::default();
        // bu imaj için bir container başlatıyoruz
        let node = docker.run(db);
        // varsayılan imaj postgresql kullanıcı ve şifresi ile 5432 portuna gidecektir
        let conn_str = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            node.get_host_port_ipv4(5432)
        );
        // test etmek istediğimiz fonksiyonelliği çağırıyoruz
        let db_conn = get_conn(conn_str).await.unwrap();
        // Sonuç beklediğimiz bir query işletiyoruz
        let query = db_conn
            .query_one(Statement::from_string(
                DatabaseBackend::Postgres,
                "SELECT 1923;".to_owned(),
            ))
            .await
            .unwrap();
        let response = query.unwrap();
        let expected: i32 = response.try_get_by_index(0).unwrap();
        assert_eq!(1923, expected);
    }
}
