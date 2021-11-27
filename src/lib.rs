#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod auth;
pub mod controllers;
pub mod cookie;
pub mod errors;
pub mod logger;
pub mod models;
pub mod oidc;
pub mod routes;
pub mod schema;
pub mod service;
pub mod types;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
