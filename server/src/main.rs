use std::process;

#[actix_web::main]
async fn main() {
    let pg_connection = match server::establish_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            process::exit(1);
        }
    };

    if let Err(error) = server::run().await {
        eprintln!("Running server error: {}", error);
        process::exit(1);
    }
}
