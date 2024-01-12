use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum BookError {
    NoBookFound,
    BookCreationFailure,
}

impl ResponseError for BookError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            BookError::NoBookFound => StatusCode::NOT_FOUND,
            BookError::BookCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
