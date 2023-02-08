use actix_web::{web, HttpResponse};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    pub foo: String,
}

pub async fn get(params: web::Query<Params>) -> HttpResponse {
    info!("params = {:?}", params);
    return HttpResponse::Ok().json(params.into_inner());
}

pub async fn post(params: web::Json<Params>) -> HttpResponse {
    info!("params = {:?}", params);
    return HttpResponse::Ok().json(params.into_inner());
}
