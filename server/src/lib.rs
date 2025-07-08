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
            .service(
                web::scope("/api")
                    .service(get_hello)
                    .service(test_json)
                    .service(get_hello),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[actix_web::get("/hello")]
async fn get_hello() -> impl Responder {
    HttpResponse::Ok().json("hello_world! (v0.1)")
}

#[actix_web::post("/json")]
async fn test_json(data: web::Json<data::User>) -> impl Responder {
    HttpResponse::Ok().body(format!("Name {} with id", data.get_name()))
}
