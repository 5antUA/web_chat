use actix_web::{HttpResponse, Responder, web};

use crate::{AppData, models::user::UserDTO, services::user_service};

#[actix_web::get("/users/{username}")]
pub async fn get_user_by_username(
    path_data: web::Path<String>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let username = path_data.into_inner();
    let pool = &app_data.pool;

    match user_service::get_user_by_id(username, pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::NotFound().body(error),
    }
}

#[actix_web::post("/users/register")]
pub async fn add_user(
    user_data: web::Json<UserDTO>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let user_data = user_data.into_inner();
    let pool = &app_data.pool;

    match user_service::add_user(&user_data, pool).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}
