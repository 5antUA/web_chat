use dotenvy::dotenv;
use std::process;

#[actix_web::main]
async fn main() {
    dotenv().ok();

    let connection_pool = match server::establish_connection().await {
        Ok(pool) => pool,
        Err(error) => {
            eprintln!("Database connection error: {}", error);
            process::exit(1);
        }
    };

    if let Err(error) = server::run(connection_pool).await {
        eprintln!("Running server error: {}", error);
        process::exit(1);
    }
}
