use log::debug;

use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::service::anime_scraper::fetch;
use crate::types::animes::Animes;
use crate::types::season::Season;

#[derive(Debug, Deserialize, Serialize)]
pub struct PathParams {
    season: Season,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseBody {
    animes: Animes,
}

#[get("/scrape/{season}")]
pub async fn get(path_params: web::Path<PathParams>) -> impl Responder {
    debug!("path params = {:?}", path_params);
    let animes = fetch(path_params.season.clone()).await;
    return HttpResponse::Ok().json(ResponseBody { animes: animes });
}
