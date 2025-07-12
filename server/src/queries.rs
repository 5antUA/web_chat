use crate::AppData;
use crate::models::user::User;

use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;

#[actix_web::get("/user")]
async fn get_user(app_data: web::Data<AppData>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let pool = &app_data.pool;
    let mut connection = pool.get().unwrap();

    let result = users
        .filter(username.eq("FantUA"))
        .select(User::as_select())
        .load(&mut connection)
        .unwrap();

    HttpResponse::Ok().body(format!("UPDATED DATA : \n{:?}", result.get(0).unwrap()))
}
