use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use log::info;
use once_cell::sync::Lazy;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub static POOL: Lazy<Pool> = Lazy::new(|| {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("Making DB connection pool...");
    return r2d2::Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(db_url))
        .expect("Failed to create pool.");
});
