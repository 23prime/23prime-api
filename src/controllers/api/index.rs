use actix_web::{HttpResponse, Responder};

pub async fn get() -> impl Responder {
    return HttpResponse::Ok().body("Hello, Anime API!!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn get_test() {
        let app = test::init_service(App::new().route("/", web::get().to(get))).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body = actix_web::test::read_body(resp).await;
        assert_eq!(body, "Hello, Anime API!!");
    }
}
