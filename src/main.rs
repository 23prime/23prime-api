use log::info;
use std::env;

use actix_web::middleware;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

use anime_api::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    if let Ok(rust_log) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", rust_log);
    } else {
        env::set_var("RUST_LOG", "debug");
    }

    info!("Log level: RUST_LOG={}", env::var("RUST_LOG").unwrap());

    return HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .configure(routes::services)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;
}
