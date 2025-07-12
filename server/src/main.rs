use dotenvy::dotenv;
use std::process;

#[actix_web::main]
async fn main() {
    dotenv().ok();

    let _ = match server::establish_connection().await {
        Ok(conn) => conn,
        Err(error) => {
            eprintln!("Database connection error: {}", error);
            process::exit(1);
        }
    };

    if let Err(error) = server::run().await {
        eprintln!("Running server error: {}", error);
        process::exit(1);
    }
}
