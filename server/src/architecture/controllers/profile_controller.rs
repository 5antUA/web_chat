use actix_web::{HttpResponse, Responder};

use crate::app_error::AppError;

pub async fn test() -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().body("not forbidden, nigga"))
}
