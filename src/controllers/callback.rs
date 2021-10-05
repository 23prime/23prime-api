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
    let token_result = token::fetch(params.code.clone()).await;
    if token_result.is_err() {
        return failed_response();
    }
    let access_token = token_result.unwrap().access_token;

    let userinfo_result = userinfo::fetch(&access_token).await;
    if userinfo_result.is_err() {
        return failed_response();
    }
    let id = userinfo_result.unwrap().sub;

    let after_login_url = env::var("AFTER_LOGIN_URL").expect("AFTER_LOGIN_URL must be set");
    let location = format!(
        "{}?id={}&access_token={}",
        after_login_url, id, access_token
    );
    return HttpResponse::Found().header(LOCATION, location).finish();
}

fn failed_response() -> HttpResponse {
    let login_failed_url = env::var("LOGIN_FAILED_URL").expect("LOGIN_FAILED_URL must be set");
    return HttpResponse::Found()
        .header(LOCATION, login_failed_url)
        .finish();
}
