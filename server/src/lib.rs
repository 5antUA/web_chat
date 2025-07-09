mod data;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
            .app_data(web::Data::new(data::AppData {
                app_name: String::from("web_chat"),
                author_name: String::from("Rostyslav"),
                version: 1,
            }))
            .service(
                web::scope("/api")
                    .service(get_hello)
                    .service(test_json)
                    .service(get_app_data),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[actix_web::get("/hello")]
async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body("hello_world! (v0.1)")
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
