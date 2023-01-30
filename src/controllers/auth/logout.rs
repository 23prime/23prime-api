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

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{test, web, App};
    use rstest::*;

    #[fixture]
    #[once]
    fn setup() {
        crate::logger::init_logger();

        env::set_var("AUTHORIZATION_ENDPOINT", "https://example.com/authorize");
        env::set_var("TOKEN_ENDPOINT", "https://example.com/oauth/token");
        env::set_var("JWKS_ENDPOINT", "https://example.com/.well-known/jwks.json");
        env::set_var("USERINFO_ENDPOINT", "https://example.com");
        env::set_var("CLIENT_ID", "XXXX");
        env::set_var("CLIENT_SECRET", "YYYY");
        env::set_var("REDIRECT_URI", "https://example.com/auth/callback");
        env::set_var("LOGOUT_ENDPOINT", "https://example.com/v2/logout");
        env::set_var("AFTER_LOGOUT_URL", "https://example.com")
    }

    #[rstest]
    #[actix_rt::test]
    async fn test_get(_setup: ()) {
        let app = test::init_service(App::new().route("/", web::get().to(get))).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::FOUND);

        let location = resp.headers().get(LOCATION).unwrap();
        assert_eq!(
            location,
            "https://example.com/v2/logout?client_id=XXXX&returnTo=https://example.com"
        );
    }
}
