extern crate diesel;

use crate::models::Anime;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ResponseBody {
    animes: Vec<Anime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PathParams {
    year: i32,
    season: Option<String>,
}

#[get("/animes/")]
pub async fn get() -> impl Responder {
    let animes = Anime::all();
    return HttpResponse::Ok().json(ResponseBody { animes: animes });
}

#[get("/animes/{year}/")]
pub async fn get_by_year(path_params: web::Path<PathParams>) -> impl Responder {
    let animes = Anime::find_by_year(path_params.year);
    return HttpResponse::Ok().json(ResponseBody { animes: animes });
}

#[get("/animes/{year}/{season}/")]
pub async fn get_by_season(path_params: web::Path<PathParams>) -> impl Responder {
    let season = &path_params.season.clone().unwrap();
    let animes = Anime::find_by_season(path_params.year, &season);
    return HttpResponse::Ok().json(ResponseBody { animes: animes });
}
