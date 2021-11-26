use log::error;

use actix_web::client::Client;
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorResponse, ServiceError};
use crate::oidc_config::OIDCConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct Userinfo {
    pub sub: String,
}

pub async fn fetch(token: &str) -> Result<Userinfo, ServiceError> {
    let userinfo_result = Client::default()
        .get(OIDCConfig::from_env().userinfo_endpoint)
        .bearer_auth(token)
        .send()
        .await;
    if userinfo_result.is_err() {
        let msg = "Failed to fetch userinfo";
        error!("{}", msg);
        return Err(ServiceError::InternalServerError(ErrorResponse::new(msg)));
    }

    let mut userinfo_response = userinfo_result.unwrap();
    if !userinfo_response.status().is_success() {
        let msg = "Failed to fetch userinfo";
        error!("{}", msg);
        return Err(ServiceError::InternalServerError(ErrorResponse::new(msg)));
    }

    let userinfo = userinfo_response.json::<Userinfo>().await.unwrap();
    return Ok(userinfo);
}
