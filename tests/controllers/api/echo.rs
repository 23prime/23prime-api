use actix_web::http::StatusCode;
use actix_web::{test, web, App};
use rstest::*;

use okkey_api::controllers::api::echo::{get, post, Params};

#[fixture]
#[once]
fn setup() {
    okkey_api::logger::init_logger();
}

#[rstest]
#[actix_rt::test]
async fn test_get(_setup: ()) {
    let app = test::init_service(App::new().route("/", web::get().to(get))).await;

    let req = test::TestRequest::get().uri("/?foo=bar").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Params = test::read_body_json(resp).await;
    assert_eq!(body.foo, "bar");
}

#[rstest]
#[actix_rt::test]
async fn test_post(_setup: ()) {
    let app = test::init_service(App::new().route("/", web::post().to(post))).await;

    let params = Params {
        foo: "bar".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/")
        .set_json(&params)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Params = test::read_body_json(resp).await;
    assert_eq!(body.foo, "bar");
}
