use log::info;
use std::env;
use std::iter;

use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{get, HttpResponse, Responder};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::errors;

#[get("/before")]
pub async fn get(session: Session) -> impl Responder {
    let authority = env::var("AUTHORITY").expect("AUTHORITY must be set");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");
    let state = generate_random_string(32);

    if session.set("state", &state).is_err() {
        return errors::failed_response();
    }

    info!("Set state = {}", state);

    let location = format!(
        "{}authorize?response_type=code&client_id={}&redirect_uri={}&scope=openid profile&state={}",
        authority, client_id, redirect_uri, state
    );
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
