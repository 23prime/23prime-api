use std::error::Error;

use actix_web::http::header::CONTENT_TYPE;
use awc::Client;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use log::{debug, error};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorResponse, ServiceError};
use crate::oidc::{OIDCConfig, OIDC_CONFIG};

#[derive(Debug, Deserialize, Serialize)]
struct TokenRequestBody {
    grant_type: String,
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
    code_verifier: String,
}

impl TokenRequestBody {
    fn default(oidc: OIDCConfig, code: String, code_verifier: String) -> Self {
        return Self {
            grant_type: "authorization_code".to_string(),
            client_id: oidc.client_id,
            client_secret: oidc.client_secret,
            code,
            redirect_uri: oidc.redirect_uri,
            code_verifier,
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
    pub acr: Option<String>,
    pub amr: Option<Vec<String>>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Deserialize, Serialize)]
struct JWKS {
    keys: Vec<JWK>,
}

#[allow(clippy::upper_case_acronyms)]
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

pub async fn fetch(code: String, code_verifier: String) -> Result<Token, ServiceError> {
    let oidc = Lazy::force(&OIDC_CONFIG);

    let token_req_body = TokenRequestBody::default(oidc.clone(), code, code_verifier);
    debug!("token_req_body = {:?}", token_req_body);

    let token_result = Client::default()
        .post(&oidc.token_endpoint)
        .append_header((CONTENT_TYPE, "application/x-www-form-urlencoded"))
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
    let splitted = id_token.split('.').collect::<Vec<&str>>();
    let header = parse_header(splitted[0]);
    let jwk_option = fetch_jwk(&header.kid).await;

    if jwk_option.is_none() {
        error!("Invalid JWT");
        return None;
    }

    let jwk = jwk_option.unwrap();
    let key_result = &DecodingKey::from_rsa_components(&jwk.n, &jwk.e);

    if key_result.is_err() {
        error!("Failed to get decode key");
        return None;
    }

    let key = key_result.clone().unwrap();
    let validation = &Validation::new(Algorithm::RS256);
    let result = decode::<Claims>(id_token, &key, validation);

    if result.is_err() {
        error!("Failed to decode ID token: {:?}", result);
        return None;
    }

    return Some(result.unwrap());
}

fn parse_header(str: &str) -> IdTokenHeader {
    let header_u8 = base64::decode_config(str, base64::URL_SAFE).unwrap();
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
    let mut response = Client::default()
        .get(&Lazy::force(&OIDC_CONFIG).jwks_endpoint)
        .send()
        .await?;
    let result = response.json::<JWKS>().await?;
    debug!("jwks = {:?}", result);
    return Ok(result);
}
