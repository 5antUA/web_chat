mod data;

use actix_web::{App, HttpResponse, HttpServer, Responder};
// use data;

pub async fn run_server() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().service(get_hello).service(test_json))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[actix_web::get("/hello_world")]
async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body("hello_world! (v2)")
}

#[actix_web::post("/json")]
async fn test_json(data: String) -> impl Responder {
    HttpResponse::Ok().body(format!("data: {}", data))
}


#[actix_web::delete("/hello_world")]
async fn del() -> impl Responder {
    HttpResponse::Ok().body("hello_world! (v2)")
}
