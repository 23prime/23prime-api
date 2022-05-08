use awc::Client;
use log::error;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorResponse, ServiceError};
use crate::oidc::OIDC_CONFIG;

#[derive(Debug, Deserialize, Serialize)]
pub struct Userinfo {
    pub sub: String,
}

pub async fn fetch(token: &str) -> Result<Userinfo, ServiceError> {
    let userinfo_result = Client::default()
        .get(&Lazy::force(&OIDC_CONFIG).userinfo_endpoint)
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
