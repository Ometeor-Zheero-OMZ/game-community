use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError
};

use derive_more::Display;

#[derive(Debug, Display)]
pub enum DataError {
    NoDataFound,
    UserCreationFailure,
    NoSuchDataFound,
    BadRequest
}

impl ResponseError for DataError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            DataError::NoDataFound => StatusCode::NOT_FOUND,
            DataError::UserCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            DataError::NoSuchDataFound => StatusCode::NOT_FOUND,
            DataError::BadRequest => StatusCode::BAD_REQUEST
        }
    }
}