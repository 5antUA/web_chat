use crate::{AppData, app_error::AppError, models::user::UserJWT, utils::jwt};

use actix_web::{
    Error,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};

pub async fn verify_jwt(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // get app_data
    let app_data = req
        .app_data::<web::Data<AppData>>()
        .ok_or(AppError::InternalServerError)?;

    // get token from request
    let token = req
        .headers()
        .get("Authorization")
        .ok_or(AppError::Unauthorized)?
        .to_str()
        .map_err(|_| AppError::Unauthorized)?
        .strip_prefix("Bearer ")
        .filter(|t| !t.trim().is_empty())
        .ok_or(AppError::Unauthorized)?;

    let data = jwt::decode_jwt::<UserJWT>(token, &app_data.jwt_secret)
        .await
        .map_err(AppError::from)?;

    next.call(req).await
}
