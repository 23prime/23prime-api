use crate::controllers::*;
use actix_web::web;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(api::index::get)
        .service(api::index::get)
        .service(api::echo::get)
        .service(api::echo::post)
        .service(api::animes::scrape::get)
        .service(api::animes::index::get)
        .service(api::animes::index::get_by_year)
        .service(api::animes::index::get_by_season)
        .service(api::animes::index::post)
        .service(api::animes::index::put)
        .service(api::animes::index::delete);
}

pub fn auth(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::callback::get);
    cfg.service(auth::before::get);
}

pub fn health_check(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check::index::get);
}
