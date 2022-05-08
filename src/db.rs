use std::env;

use log::info;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

pub static POOL: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_pool() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("Creating DB connection pool...");

    return Database::connect(&db_url)
        .await
        .expect("Failed to create DB connection pool");
}
