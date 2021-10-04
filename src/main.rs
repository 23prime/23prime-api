use std::env;

use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;

use anime_api::auth;
use anime_api::logger;
use anime_api::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    logger::set_logger();

    let host = env::var("ACTIX_HOST").unwrap();
    let port = env::var("ACTIX_PORT").unwrap();

    return HttpServer::new(move || {
        App::new()
            .wrap(HttpAuthentication::bearer(auth::validator))
            .wrap(Logger::default())
            .wrap(NormalizePath::default())
            .configure(routes::services)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await;
}
