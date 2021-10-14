use log::{debug, error};
use std::env;
use std::error::Error;

use actix_web::client::Client;
use actix_web::http::header::CONTENT_TYPE;
use base64::{decode_config, URL_SAFE};
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
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
    pub scope: String,
    pub expires_in: i64,
    pub token_type: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct IdTokenHeader {
    alg: String,
    typ: String,
    kid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // by scope=openid
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub iat: i64,
    pub exp: i64,

    // by scope=profile
    pub nickname: String,
    pub name: String,
    pub picture: String,
    pub updated_at: String,
    pub acr: String,
    pub amr: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JWKS {
    keys: Vec<JWK>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JWK {
    alg: String,
    kty: String,
    r#use: String,
    n: String,
    e: String,
    kid: String,
    x5t: String,
    x5c: Vec<String>,
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

pub async fn validate_id_token(id_token: &str) -> Option<TokenData<Claims>> {
    debug!("id_token = {:?}", id_token);
    let splited = id_token.split(".").collect::<Vec<&str>>();
    let header = parse_header(splited[0]);

    if let Some(jwk) = fetch_jwk(&header.kid).await {
        let key = &DecodingKey::from_rsa_components(&jwk.n, &jwk.e);
        let validation = &Validation::new(Algorithm::RS256);
        if let Ok(result) = decode::<Claims>(&id_token, key, validation) {
            debug!("result = {:?}", result);
            return Some(result);
        }
    }

    return None;
}

fn parse_header(str: &str) -> IdTokenHeader {
    let header_u8 = decode_config(str, URL_SAFE).unwrap();
    let result = serde_json::from_slice(&header_u8).unwrap();
    debug!("header = {:?}", result);
    return result;
}

async fn fetch_jwk(kid: &str) -> Option<JWK> {
    if let Ok(jwks) = fetch_jwks().await {
        return jwks.keys.into_iter().find(|j| j.kid == kid);
    }
    return None;
}

async fn fetch_jwks() -> Result<JWKS, Box<dyn Error>> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let url = &format!("{}{}", authority.as_str(), ".well-known/jwks.json");
    let mut response = Client::default().get(url).send().await?;
    let result = response.json::<JWKS>().await?;
    debug!("jwks = {:?}", result);
    return Ok(result);
}
