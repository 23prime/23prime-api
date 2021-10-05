use std::env;

use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::service::{token, userinfo};

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseBody {
    access_token: String,
}

#[get("/callback/")]
pub async fn get(params: web::Query<Params>) -> impl Responder {
    let access_token = token::fetch(params.code.clone())
        .await
        .unwrap()
        .access_token;
    let id = userinfo::fetch(&access_token).await.unwrap().sub;
    let after_login_url = env::var("AFTER_LOGIN_URL").expect("AFTER_LOGIN_URL must be set");
    let location = format!(
        "{}?id={}&access_token={}",
        after_login_url, id, access_token
    );
    return HttpResponse::Found().header(LOCATION, location).finish();
}
