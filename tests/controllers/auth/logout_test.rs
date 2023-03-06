use std::env;

use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::{test, web, App};
use rstest::*;

use okkey_api::controllers::auth::logout::get;

#[fixture]
#[once]
fn setup() {
    okkey_api::logger::init_logger();

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
