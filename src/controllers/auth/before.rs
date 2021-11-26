use log::info;
use std::iter;

use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{get, HttpResponse, Responder};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};

use crate::errors;
use crate::oidc_config::OIDCConfig;

#[get("/before")]
pub async fn get(session: Session) -> impl Responder {
    let oidc_config = OIDCConfig::from_env();
    let state = generate_random_string(32);

    if session.set("state", &state).is_err() {
        return errors::failed_response();
    }

    let code_verifier = generate_random_string(64);
    let code_challenge = base64::encode_config(
        Sha256::digest(&code_verifier.as_bytes()),
        base64::URL_SAFE_NO_PAD,
    );

    if session.set("code_verifier", &code_verifier).is_err() {
        return errors::failed_response();
    }

    let redirect_params = vec![
        "response_type=code".to_string(),
        format!("client_id={}", oidc_config.client_id),
        format!("redirect_uri={}", oidc_config.redirect_uri),
        "scope=openid profile".to_string(),
        format!("state={}", state),
        format!("code_challenge={}", code_challenge),
        "code_challenge_method=S256".to_string(),
    ];

    let location = format!(
        "{}?{}",
        oidc_config.authorization_endpoint,
        redirect_params.into_iter().collect::<Vec<_>>().join("&")
    );
    info!("Login URL generated => {}", location);
    return HttpResponse::Found().header(LOCATION, location).finish();
}

fn generate_random_string(len: usize) -> String {
    let mut rng = thread_rng();
    return iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect();
}
