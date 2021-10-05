use log::{debug, error};
use std::env;

use actix_web::client::Client;
use actix_web::http::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorResponse, ServiceError};

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
pub struct Token {
    pub access_token: String,
    pub id_token: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenError {
    error: String,
    error_description: String,
}

pub async fn fetch(code: String) -> Result<Token, ServiceError> {
    let authority = env::var("AUTHORITY").expect("AUTHORITY must be set");
    let url = &format!("{}{}", authority.as_str(), "oauth/token");
    debug!("url = {:?}", url);

    let token_req_body = TokenRequestBody::default(code);
    debug!("token_req_body = {:?}", token_req_body);

    let token_result = Client::default()
        .post(url)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send_form(&token_req_body)
        .await;
    debug!("token_result = {:?}", token_result);

    if token_result.is_err() {
        return Err(ServiceError::InternalServerError(ErrorResponse::new(
            "Failed to fetch token",
        )));
    }

    let mut token_response = token_result.unwrap();
    if !token_response.status().is_success() {
        error!(
            "token_response = {:?}",
            token_response.json::<TokenError>().await
        );
        return Err(ServiceError::InternalServerError(ErrorResponse::new(
            "Failed to fetch token",
        )));
    }
    debug!("token_response = {:?}", token_response);

    let token = token_response.json::<Token>().await.unwrap();
    return Ok(token);
}
