use crate::{AppData, models::user::UserDTO, services::user_service};

use crate::app_error::AppError;
use actix_web::{HttpResponse, Responder, web};

pub async fn get_user(
    path_data: web::Path<String>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, AppError> {
    let username = path_data.into_inner();
    let pool = &app_data.pool;

    let received_user = user_service::get_user(username, pool).await?;

    Ok(HttpResponse::Ok().json(received_user))
}

pub async fn add_user(
    user_data: web::Json<UserDTO>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let user_data = user_data.into_inner();
    let pool = &app_data.pool;

    match user_service::add_user(&user_data, pool).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(error) => HttpResponse::Conflict().body(error.to_string()),
    }
}
