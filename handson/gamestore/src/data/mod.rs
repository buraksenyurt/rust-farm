use crate::app_settings::AppSettings;
use sea_orm::*;

pub(super) async fn connect(settings: &AppSettings) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(format!(
        "mysql://{}:{}@{}:{}/{}",
        settings.db_username,
        settings.db_password,
        settings.db_host,
        settings.db_port,
        settings.db_database
    ));
    options.sqlx_logging(false);
    Database::connect(options).await
}
