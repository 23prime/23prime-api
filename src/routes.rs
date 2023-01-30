use crate::controllers::*;
use actix_web::web::{delete, get, post, put, resource, ServiceConfig};

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(get().to(api::index::get)))
        .service(
            resource("/echo")
                .route(get().to(api::echo::get))
                .route(post().to(api::echo::post)),
        )
        .service(resource("/animes/scrape/{season}").route(get().to(api::animes::scrape::get)))
        .service(
            resource("/animes")
                .route(get().to(api::animes::index::get))
                .route(post().to(api::animes::index::post))
                .route(put().to(api::animes::index::put))
                .route(delete().to(api::animes::index::delete)),
        )
        .service(resource("/animes/{year}").route(get().to(api::animes::index::get_by_year)))
        .service(
            resource("/animes/{year}/{season}").route(get().to(api::animes::index::get_by_season)),
        );
}

pub fn auth(cfg: &mut ServiceConfig) {
    cfg.service(resource("/callback").route(get().to(auth::callback::get)))
        .service(resource("/before").route(get().to(auth::before::get)))
        .service(resource("/logout").route(get().to(auth::logout::get)));
}

pub fn health_check(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(get().to(health_check::index::get)));
}
