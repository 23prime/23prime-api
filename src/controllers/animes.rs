extern crate diesel;

use crate::models::Anime;
use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseBody {
    animes: Vec<Anime>,
}

#[get("/animes/")]
pub async fn get() -> impl Responder {
    let animes = Anime::all();
    return HttpResponse::Ok().json(ResponseBody { animes: animes });
}
