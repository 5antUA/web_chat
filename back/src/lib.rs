use actix_web::{App, HttpResponse, HttpServer, Responder};

pub async fn run_server() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().service(get_hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[actix_web::get("/hello_world")]
async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body("hello_world!")
}
