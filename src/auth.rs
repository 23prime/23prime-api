use log::{debug, error, info};

use actix_web::client::Client;
use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

use crate::errors::{ErrorResponse, ServiceError};

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
    let url = &format!("{}{}", authority.as_str(), "userinfo");

    let userinfo_result = Client::default().get(url).bearer_auth(token).send().await;

    if userinfo_result.is_err() {
        let msg = "Failed to fetch userinfo";
        error!("{}", msg);
        return Err(ServiceError::InternalServerError(ErrorResponse::new(msg)));
    }

    let userinfo_response = userinfo_result.unwrap();
    debug!("userinfo_response = {:?}", userinfo_response);
    return Ok(userinfo_response.status().is_success());
}
