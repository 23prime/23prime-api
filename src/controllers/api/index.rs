use actix_web::{HttpResponse, Responder};

pub async fn get() -> impl Responder {
    return HttpResponse::Ok().body("Hello, Anime API!!");
}
