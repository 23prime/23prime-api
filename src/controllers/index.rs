use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok().body("Hello, Anime scraper!!")
}
