use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("400 Bad Request [rost]")]
    BadRequest,

    #[error("401 Unauthorized [rost]")]
    Unauthorized,

    #[error("403 Forbidden [rost]")]
    Forbidden,

    #[error("404 Not Found [rost]")]
    NotFound,

    #[error("405 Method Not Allowed [rost]")]
    MethodNotAllowed,

    #[error("409 Conflict [rost]")]
    Conflict,

    #[error("422 Unprocessable Entity [rost]")]
    UnprocessableEntity,

    #[error("500 Internal Server Error [rost]")]
    InternalServerError,

    #[error("501 Not Implemented [rost]")]
    NotImplemented,

    #[error("502 Bad Gateway [rost]")]
    BadGateway,

    #[error("503 Service Unavailable [rost]")]
    ServiceUnavailable,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,

            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            AppError::BadGateway => StatusCode::BAD_GATEWAY,
            AppError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}
