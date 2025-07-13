pub mod models;
mod queries;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub struct AppData {
    pub app_name: String,
    pub pool: Pool<Postgres>,
}

pub async fn run(pool: Pool<Postgres>) -> Result<(), std::io::Error> {
    let app_data = web::Data::new(AppData {
        app_name: String::from("web_chat"),
        pool,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
            .app_data(app_data.clone())
            .service(web::scope("/api").service(queries::get_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub async fn establish_connection() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pg_pool)
}
