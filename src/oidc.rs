use std::env;

use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct OIDCConfig {
    pub authorization_endpoint: String,
    pub userinfo_endpoint: String,
    pub token_endpoint: String,
    pub jwks_endpoint: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl OIDCConfig {
    pub fn from_env() -> Self {
        return OIDCConfig {
            authorization_endpoint: env::var("AUTHORIZATION_ENDPOINT")
                .expect("AUTHORIZATION_ENDPOINT must be set"),
            token_endpoint: env::var("TOKEN_ENDPOINT").expect("TOKEN_ENDPOINT must be set"),
            jwks_endpoint: env::var("JWKS_ENDPOINT").expect("JWKS_ENDPOINT must be set"),
            userinfo_endpoint: env::var("USERINFO_ENDPOINT")
                .expect("USERINFO_ENDPOINT must be set"),
            client_id: env::var("CLIENT_ID").expect("CLIENT_ID must be set"),
            client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set"),
            redirect_uri: env::var("REDIRECT_URI").expect("REDIRECT_URI must be set"),
        };
    }
}

pub static OIDC_CONFIG: Lazy<OIDCConfig> = Lazy::new(OIDCConfig::from_env);
