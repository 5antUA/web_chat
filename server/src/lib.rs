pub mod data;
// pub mod models;
pub mod queries;
pub mod schema;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

use diesel::prelude::*;
use std::env;

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
                author_name: String::from("Rostikus"),
                version: 1,
            }))
            .service(
                web::scope("/api")
                    .service(queries::get_hello)
                    .service(queries::test_json)
                    .service(queries::get_app_data),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub async fn establish_connection() -> Result<PgConnection, Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL")?;
    let pg_connection = PgConnection::establish(&database_url)?;

    Ok(pg_connection)
}
