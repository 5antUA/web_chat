pub mod models;
mod queries;
pub mod schema;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};

type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub struct AppData {
    pub app_name: String,
    pub pool: ConnectionPool,
}

pub async fn run(pool: ConnectionPool) -> Result<(), std::io::Error> {
    let app_data = web::Data::new(AppData {
        app_name: String::from("web_chat"),
        pool
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
            .service(
                web::scope("/api")
                    .service(queries::get_user)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub async fn establish_connection() -> Result<ConnectionPool, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;

    Ok(pool)
}
