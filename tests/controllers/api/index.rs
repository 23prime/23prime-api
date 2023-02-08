use actix_web::http::StatusCode;
use actix_web::{test, web, App};
use rstest::*;

use okkey_api::controllers::api::index::get;

#[fixture]
#[once]
fn setup() {
    okkey_api::logger::init_logger();
}

#[rstest]
#[actix_rt::test]
async fn test_get(_setup: ()) {
    let app = test::init_service(App::new().route("/", web::get().to(get))).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body = actix_web::test::read_body(resp).await;
    assert_eq!(body, "Hello, Anime API!!");
}
