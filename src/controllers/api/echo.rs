use actix_web::{web, HttpResponse};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    foo: String,
}

pub async fn get(params: web::Query<Params>) -> HttpResponse {
    info!("params = {:?}", params);
    return HttpResponse::Ok().json(params.into_inner());
}

pub async fn post(params: web::Json<Params>) -> HttpResponse {
    info!("params = {:?}", params);
    return HttpResponse::Ok().json(params.into_inner());
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{test, web, App};
    use rstest::*;

    #[fixture]
    #[once]
    fn setup() {
        crate::logger::init_logger();
    }

    #[rstest]
    #[actix_rt::test]
    async fn get_test(_setup: ()) {
        let app = test::init_service(App::new().route("/", web::get().to(get))).await;

        let req = test::TestRequest::get().uri("/?foo=bar").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: Params = test::read_body_json(resp).await;
        assert_eq!(body.foo, "bar");
    }

    #[rstest]
    #[actix_rt::test]
    async fn post_test(_setup: ()) {
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
}
