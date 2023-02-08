use actix_web::HttpResponse;

pub async fn get() -> HttpResponse {
    return HttpResponse::Ok().body("OK");
}
