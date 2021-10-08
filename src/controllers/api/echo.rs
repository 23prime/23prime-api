use log::info;

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    foo: String,
}

#[get("/echo/")]
pub async fn get(params: web::Query<Params>) -> impl Responder {
    info!("params = {:?}", params);
    return HttpResponse::Ok().json(params.into_inner());
}

#[post("/echo/")]
pub async fn post(params: web::Json<Params>) -> impl Responder {
    info!("params = {:?}", params);
    return HttpResponse::Ok().json(params.into_inner());
}
