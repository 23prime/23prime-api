use actix_web::error::ResponseError;
use actix_web::HttpResponse;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    reason: String,
}

impl ErrorResponse {
    pub fn new<'a>(reason: &str) -> Self {
        return Self {
            reason: reason.to_string(),
        };
    }
}

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError(ErrorResponse),

    #[display(fmt = "Bad Request")]
    BadRequest(ErrorResponse),

    #[display(fmt = "Unauthorized")]
    Unauthorized(ErrorResponse),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::InternalServerError(ref body) => HttpResponse::InternalServerError().json(body),
            Self::BadRequest(ref body) => HttpResponse::BadRequest().json(body),
            Self::Unauthorized(ref body) => HttpResponse::Unauthorized().json(body),
        }
    }
}
