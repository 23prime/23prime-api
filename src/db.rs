use std::env;

use log::info;
use sea_orm::{Database, DatabaseConnection};

pub async fn get_db() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let result = Database::connect(&db_url)
        .await
        .expect("Failed to create DB connection pool");

    info!("DB connection has been created: {:?}", result);
    return result;
}
