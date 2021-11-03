extern crate diesel;

use actix_web::{get, HttpResponse, Responder};

#[get("")]
pub async fn get() -> impl Responder {
    return HttpResponse::Ok().body("Hello, Anime API!!");
}
