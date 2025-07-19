pub mod architecture;
pub mod models;
pub mod utils;

pub mod app_error;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::architecture::{controllers, middlewares, repositories, routers, services};

pub struct AppData {
    pub jwt_secret: String,
    pub pool: PgPool,
}

pub async fn run(pool: PgPool) -> Result<()> {
    let app_data = web::Data::new(AppData {
        jwt_secret: std::env::var("JWT_SECRET")?,
        pool,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(app_data.clone())
            .service(web::scope("/api").configure(routers::configure_routes_all))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    anyhow::Ok(())
}

pub async fn establish_connection() -> Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL")?;
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pg_pool)
}
