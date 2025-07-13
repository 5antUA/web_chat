use actix_web::{HttpResponse, Responder, web};

use crate::{AppData, models::user::UserDTO, services::user_service};

#[actix_web::get("/users/{id}")]
pub async fn get_user_by_id(
    path_data: web::Path<i32>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let user_id = path_data.into_inner();
    let pool = &app_data.pool;

    match user_service::get_user_by_id(pool, user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::NotFound().body(error),
    }
}

#[actix_web::post("/users/register")]
pub async fn add_user(
    user_dto: web::Json<UserDTO>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let pool = &app_data.pool;
    let data = &user_dto.into_inner();
    println!("{:?}", data);

    match user_service::add_user(pool, data).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::NotFound().body(error),
    }
}
