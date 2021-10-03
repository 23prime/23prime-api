use log::debug;

use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::service::anime::fetch_all;
use crate::types::season::Season;

#[derive(Debug, Deserialize, Serialize)]
pub struct PathParams {
    year: u64,
    season: Season,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryParams {
    foo: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseBody {
    animes: Vec<String>,
}

#[get("/scrape/{year}/{season}/")]
pub async fn get(
    path_params: web::Path<PathParams>,
    query_params: web::Query<QueryParams>,
) -> impl Responder {
    debug!("path params = {:?}", path_params);
    debug!("query params = {:?}", query_params);

    let animes = fetch_all(&path_params.season).await;
    return HttpResponse::Ok().json(ResponseBody { animes: animes });
}
