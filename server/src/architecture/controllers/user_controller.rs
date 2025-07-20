use crate::utils::jwt::JsonWebToken;
use crate::{AppData, models::user::UserInfo, services::user_service};

use crate::app_error::AppError;
use actix_web::{HttpResponse, Responder, web};

// get user
pub async fn get_user(
    user: web::Path<String>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, AppError> {
    let username = user.into_inner();
    let app_data = app_data.into_inner();

    let received_user = user_service::get_user(username, &app_data).await?;
    Ok(HttpResponse::Ok().json(received_user))
}

// register user
pub async fn register_user(
    user_data: web::Json<UserInfo>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, AppError> {
    let mut user_data = user_data.into_inner();
    let app_data = app_data.into_inner();

    let registered_user = user_service::register_user(&mut user_data, &app_data).await?;
    Ok(HttpResponse::Created().json(registered_user))
}

// login user (sex)
pub async fn login_user(
    user_data: web::Json<UserInfo>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, AppError> {
    let user_data = user_data.into_inner();
    let app_data = app_data.into_inner();

    let jwt = user_service::login_user(&user_data, &app_data).await?;
    Ok(HttpResponse::Ok().json(JsonWebToken { token: jwt }))
}
