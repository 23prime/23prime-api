use std::env;

use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use log::info;
use once_cell::sync::Lazy;

use crate::oidc::OIDC_CONFIG;

pub async fn get() -> HttpResponse {
    let oidc = &Lazy::force(&OIDC_CONFIG);

    let redirect_params = vec![
        format!("client_id={}", oidc.client_id),
        format!("returnTo={}", get_after_logout_url()),
    ];

    let location = format!(
        "{}?{}",
        oidc.logout_endpoint,
        redirect_params.into_iter().collect::<Vec<_>>().join("&")
    );
    info!("Logout URL generated => {}", location);

    return HttpResponse::Found()
        .append_header((LOCATION, location))
        .finish();
}

fn get_after_logout_url() -> String {
    return env::var("AFTER_LOGOUT_URL").expect("AFTER_LOGOUT_URL must be set");
}
