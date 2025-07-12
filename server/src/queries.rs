use crate::data;
use actix_web::{HttpResponse, Responder, web};

#[actix_web::get("/user")]
async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body("some user")
}

#[actix_web::post("/json")]
async fn test_json(data: web::Json<data::User>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Name {} with id {}",
        data.get_name(),
        data.get_id()
    ))
}

#[actix_web::get("/data")]
async fn get_app_data(data: web::Data<data::AppData>) -> impl Responder {
    let app_data = format!(
        "app: {}, author: {}, version: {}",
        data.app_name, data.author_name, data.version
    );

    HttpResponse::Ok().body(app_data)
}
