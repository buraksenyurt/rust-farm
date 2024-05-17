use crate::db::DbContext;
use std::sync::Mutex;

pub struct AppState {
    pub db_context: Mutex<DbContext>,
}
