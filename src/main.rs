use actix_web::middleware::Logger;

use actix_web::{App, HttpServer};

use anime_scraper::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::services)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
