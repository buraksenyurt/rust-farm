use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;
const HOST: &str = "localhost";
const ROOT_DB: &str = "postgres";
const USER: &str = "scoth";
const PASSWORD: &str = "tiger";

pub async fn init() -> Result<Db, sqlx::Error> {
    let con_str = format!("postrgres://{}:{}@{}/{}", USER, PASSWORD, HOST, ROOT_DB);
    PgPoolOptions::new()
        .max_connections(1)
        .connect(&con_str)
        .await
}

#[cfg(test)]
mod tests {
    use crate::model::database::init;

    #[tokio::test]
    async fn model_database_init() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;
        Ok(())
    }
}
