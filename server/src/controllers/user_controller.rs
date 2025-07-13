use actix_web::{HttpResponse, Responder, web};

use crate::{AppData, services::user_service};

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
