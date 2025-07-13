use crate::{AppData, models::user::User};

use actix_web::{web, HttpResponse, Responder};

#[actix_web::get("/user")]
async fn get_user(app_data: web::Data<AppData>) -> impl Responder {
    let pool = &app_data.pool;

    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id=1")
        .fetch_one(pool)
        .await
    {
        Ok(usr) => usr,
        Err(e) => {
            let e_type = format!("Error: {}", e.to_string());
            return HttpResponse::NotFound().body(e_type);
        }
    };

    HttpResponse::Ok().body(format!("{:?}", user))
}
