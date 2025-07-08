mod data;

use std::process;

#[actix_web::main]
async fn main() {
    if let Err(error) = server::run_server().await {
        eprintln!("Running server error: {}", error);
        process::exit(1);
    }
}
