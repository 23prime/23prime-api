use actix_web::{web, HttpResponse, Responder};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::service::anime_scraper::fetch;
use crate::types::animes::StrictAnimes;
use crate::types::season::Season;

#[derive(Debug, Deserialize, Serialize)]
pub struct PathParams {
    season: Season,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseBody {
    animes: StrictAnimes,
}

pub async fn get(path_params: web::Path<PathParams>) -> impl Responder {
    debug!("path params = {:?}", path_params);
    let animes = fetch(path_params.season.clone()).await;
    return HttpResponse::Ok().json(ResponseBody { animes });
}
