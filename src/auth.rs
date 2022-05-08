use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use awc::Client;
use log::{debug, error, info};
use once_cell::sync::Lazy;

use crate::errors::{ErrorResponse, ServiceError};
use crate::oidc::OIDC_CONFIG;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req.app_data::<Config>().cloned().unwrap_or_default();

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
    let userinfo_result = Client::default()
        .get(&Lazy::force(&OIDC_CONFIG).userinfo_endpoint)
        .bearer_auth(token)
        .send()
        .await;

    if userinfo_result.is_err() {
        let msg = "Failed to fetch userinfo";
        error!("{}: {:?}", msg, userinfo_result);
        return Err(ServiceError::InternalServerError(ErrorResponse::new(msg)));
    }

    let userinfo_response = userinfo_result.unwrap();
    debug!("userinfo_response = {:?}", userinfo_response);
    return Ok(userinfo_response.status().is_success());
}
