use crate::controllers::*;
use actix_web::web;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(api::index::get)
        .service(api::index::get)
        .service(api::echo::get)
        .service(api::echo::post)
        .service(api::scrape::get)
        .service(api::animes::get)
        .service(api::animes::get_by_year)
        .service(api::animes::get_by_season)
        .service(api::animes::post)
        .service(api::animes::put)
        .service(api::animes::delete);
}

pub fn auth(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::callback::get);
}

pub fn health_check(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check::index::get);
}
