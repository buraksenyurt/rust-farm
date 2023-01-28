use crate::error::db_error::DbError;
use crate::error::db_error::DbError::DbInitError;
use crate::error::handler::DbResult;
use mobc::{Connection, Error, Pool};
use mobc_postgres::PgConnectionManager;
use std::convert::Infallible;
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, NoTls};
use warp::Filter;

pub type Conn = Connection<PgConnectionManager<NoTls>>;
pub type ConnPool = Pool<PgConnectionManager<NoTls>>;

const CONN_POOL_TIMEOUT: u64 = 15;
const CONN_POOL_MAX_OPEN: u64 = 32;
const CONN_POOL_MAX_IDLE: u64 = 8;

pub fn create_conn_pool() -> Result<ConnPool, Error<DbError>> {
    let config = Config::from_str("postgres://scoth:tiger@127.0.0.1:5432/postgres")
        .expect("bağlantı bilgisi okunamadı");
    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(CONN_POOL_MAX_OPEN)
        .max_idle(CONN_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(CONN_POOL_TIMEOUT)))
        .build(manager))
}

pub async fn get_db_conn(conn_pool: &ConnPool) -> Result<Conn, DbError> {
    conn_pool.get().await.map_err(DbError::ConnPoolError)
}

pub async fn init_db(conn_pool: &ConnPool) -> DbResult<()> {
    let seed_data_sql = fs::read_to_string("./seed_data.sql").expect("dosya okunamadı");
    let con = get_db_conn(conn_pool).await?;
    con.batch_execute(seed_data_sql.as_str())
        .await
        .map_err(DbInitError)?;
    Ok(())
}

pub fn add_with_db(
    conn_pool: Pool<PgConnectionManager<NoTls>>,
) -> impl Filter<Extract = (ConnPool,), Error = Infallible> + Clone {
    warp::any().map(move || conn_pool.clone())
}
