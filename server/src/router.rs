use crate::{AppData, models::user::User};

use actix_web::{HttpResponse, Responder, web};

#[actix_web::get("/user/{username}")]
async fn get_user_by_id(
    path_data: web::Path<String>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let username = path_data.into_inner();
    let pool = &app_data.pool;

    let query_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await;

    match query_result {
        Ok(user) => return HttpResponse::Ok().json(user),
        Err(e) => {
            let e_type = format!("ERROR (NOT FOUND): \n\t {}", e.to_string());
            return HttpResponse::NotFound().body(e_type);
        }
    }
}
