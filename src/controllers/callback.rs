use log::{debug, error};
use std::env;

use actix_web::client::Client;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::errors::ErrorResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseBody {
    access_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenRequestBody {
    grant_type: String,
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
}

impl TokenRequestBody {
    fn default(code: String) -> Self {
        return Self {
            grant_type: "authorization_code".to_string(),
            client_id: env::var("CLIENT_ID").expect("CLIENT_ID must be set"),
            client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set"),
            code: code,
            redirect_uri: env::var("REDIRECT_URI").expect("REDIRECT_URI must be set"),
        };
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenResponse {
    access_token: String,
    id_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenErrorResponse {
    error: String,
    error_description: String,
}

#[get("/callback/")]
pub async fn get(params: web::Query<Params>) -> impl Responder {
    let authority = env::var("AUTHORITY").expect("AUTHORITY must be set");
    let url = &format!("{}{}", authority.as_str(), "oauth/token");
    debug!("url = {:?}", url);

    let token_req_body = TokenRequestBody::default(params.code.clone());
    debug!("token_req_body = {:?}", token_req_body);

    let token_result = Client::default()
        .post(url)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send_form(&token_req_body)
        .await;
    debug!("token_result = {:?}", token_result);

    if token_result.is_err() {
        return HttpResponse::InternalServerError()
            .json(ErrorResponse::new("Failed to fetch token"));
    }

    let mut token_response = token_result.unwrap();
    if !token_response.status().is_success() {
        error!(
            "token_response = {:?}",
            token_response.json::<TokenErrorResponse>().await
        );
        return HttpResponse::InternalServerError()
            .json(ErrorResponse::new("Failed to fetch token"));
    }
    debug!("token_response = {:?}", token_response);

    let access_token = token_response
        .json::<TokenResponse>()
        .await
        .unwrap()
        .access_token;

    return HttpResponse::Ok().json(ResponseBody {
        access_token: access_token,
    });
}
