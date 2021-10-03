use actix_web::{post, HttpResponse, Responder};

#[post("/echo")]
pub async fn post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
