use actix_web::middleware;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

use anime_api::logger;
use anime_api::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    logger::set_logger();

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
