extern crate diesel;

use log::info;

use crate::models::{Anime, NewAnime};
use actix_web::{get, post, web, HttpResponse, Responder};
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BodyParams {
    animes: Vec<NewAnime>,
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

#[post("/animes/")]
pub async fn post(body_params: web::Json<BodyParams>) -> impl Responder {
    let new_animes = &body_params.animes;
    info!("Try create new_animes: {:?}", new_animes);
    let created_animes = Anime::create(new_animes);

    if created_animes.is_err() {
        info!("Failed to create new animes: {:?}", created_animes);
        return HttpResponse::BadRequest().finish();
    }

    return HttpResponse::Ok().json(ResponseBody {
        animes: created_animes.unwrap(),
    });
}
