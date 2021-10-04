use log::{debug, error, info};

use actix_web::client::{Client, JsonPayloadError};
use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match validate_token(credentials.token()).await {
        Ok(result) => {
            info!("validate_token => {}", result);
            if result {
                return Ok(req);
            }
            return Err(AuthenticationError::from(config).into());
        }
        Err(_) => {
            info!("validate_token => error");
            return Err(AuthenticationError::from(config).into());
        }
    }
}

async fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    debug!("authority = {:?}", authority);
    let uri = &format!("{}{}", authority.as_str(), ".well-known/jwks.json");

    let jwks = match fetch_jwks(uri).await {
        Ok(jr) => jr,
        Err(_) => {
            error!("Failed to fetch JWK");
            return Err(ServiceError::JWKSFetchError);
        }
    };
    debug!("jwks = {:?}", jwks);

    let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];

    let kid = match token_kid(&token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => {
            error!("Failed to fetch JWK");
            return Err(ServiceError::JWKSFetchError);
        }
    };

    let jwk = match jwks.find(&kid) {
        Some(o) => o,
        None => {
            error!("Could not find kid from JWKs");
            return Err(ServiceError::JWKSFetchError);
        }
    };

    let result = validate(token, jwk, validations);
    Ok(result.is_ok())
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, JsonPayloadError> {
    let mut response = Client::default().get(uri).send().await.unwrap();
    let result = response.json::<JWKS>().await;
    return result;
}
