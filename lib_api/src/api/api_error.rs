use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(String),
    #[error("Patient not found")]
    NotFound,
    #[error("Patient was changed")]
    Conflict,
    #[error("Unexpected error")]
    InternalServerError,
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Conflict => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }
}
