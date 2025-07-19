use crate::{AppData, models::user::UserDTO, services::user_service};

use crate::app_error::AppError;
use actix_web::{HttpResponse, Responder, web};

// get user
pub async fn get_user(
    user: web::Path<String>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, AppError> {
    let username = user.into_inner();
    let pool = &app_data.pool;

    let received_user = user_service::get_user(username, pool).await?;
    Ok(HttpResponse::Ok().json(received_user))
}

// register user
pub async fn register_user(
    user_data: web::Json<UserDTO>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, AppError> {
    let mut user_data = user_data.into_inner();
    let pool = &app_data.pool;

    let received_user = user_service::register_user(&mut user_data, pool).await?;
    Ok(HttpResponse::Created().json(received_user))
}

// login user (sex)
pub async fn login_user(
    user_data: web::Json<UserDTO>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, AppError> {
    let user_data = user_data.into_inner();
    let pool = &app_data.pool;

    let result = user_service::login_user(&user_data, &pool).await?;
    Ok(HttpResponse::Ok().json(result))
}
