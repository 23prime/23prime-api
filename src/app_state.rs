use sea_orm::DatabaseConnection;

use crate::db;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn init() -> Self {
        return Self {
            db: db::get_db().await,
        };
    }
}
