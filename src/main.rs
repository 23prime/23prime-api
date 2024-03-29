use std::env;

use actix_cors::Cors;
use actix_web::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use actix_web::middleware::Logger;
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use log::info;

use okkey_api::app_state::AppState;
use okkey_api::auth;
use okkey_api::cookie;
use okkey_api::logger;
use okkey_api::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    logger::init_logger();
    info!("LOG_LEVEL={}", env::var("LOG_LEVEL").unwrap());

    let host = env::var("ACTIX_HOST").unwrap();
    let port = env::var("ACTIX_PORT").unwrap();

    let app_state = AppState::init().await;

    return HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTION"])
            .allowed_headers(vec![AUTHORIZATION, ACCEPT, CONTENT_TYPE])
            .max_age(86400);

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(cors)
            .service(
                web::scope("/api")
                    .wrap(HttpAuthentication::bearer(auth::validator))
                    .configure(routes::api),
            )
            .service(
                web::scope("/auth")
                    .wrap(cookie::config())
                    .configure(routes::auth),
            )
            .service(web::scope("/health_check").configure(routes::health_check))
            .service(actix_files::Files::new("/", "static").show_files_listing())
    })
    .bind(format!("{host}:{port}"))?
    .run()
    .await;
}
