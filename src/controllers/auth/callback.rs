use std::env;

use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};

use crate::errors;
use crate::service::token;

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    code: String,
    state: String,
}

pub async fn get(params: web::Query<Params>, session: Session) -> impl Responder {
    if !validate_state(&params.state, &session) {
        return errors::failed_response();
    }

    let code_verifier = get_code_verifier(&session);
    if code_verifier.is_none() {
        return errors::failed_response();
    }

    let token_result = token::fetch(params.code.clone(), code_verifier.unwrap()).await;
    if token_result.is_err() {
        return errors::failed_response();
    }
    let token = token_result.unwrap();
    let token_data = token::validate_id_token(&token.id_token).await;
    if token_data.is_none() {
        return errors::failed_response();
    }

    let claims = token_data.unwrap().claims;

    let after_login_url = env::var("AFTER_LOGIN_URL").expect("AFTER_LOGIN_URL must be set");
    let location = format!(
        "{}?id={}&name={}&access_token={}",
        after_login_url, claims.sub, claims.name, token.access_token,
    );
    return HttpResponse::Found()
        .append_header((LOCATION, location))
        .finish();
}

fn validate_state(param_state: &str, session: &Session) -> bool {
    let session_state_result = session.get::<String>("state");

    if session_state_result.is_err() {
        return false;
    }

    if let Some(session_state) = session_state_result.unwrap() {
        info!(
            "states: in param = {}, in session = {}",
            param_state, session_state
        );
        let result = param_state == session_state;
        return result;
    }

    return false;
}

fn get_code_verifier(session: &Session) -> Option<String> {
    let result = session.get::<String>("code_verifier");

    if result.is_err() {
        return None;
    }

    return result.unwrap();
}
