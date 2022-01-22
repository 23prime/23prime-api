use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use once_cell::sync::Lazy;

static DATABASE_URL: Lazy<String> =
    Lazy::new(|| env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

pub fn establish_connection() -> PgConnection {
    PgConnection::establish(&DATABASE_URL).expect(&format!(
        "Error connecting to {}",
        Lazy::force(&DATABASE_URL)
    ))
}
