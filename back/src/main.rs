use std::process;

#[actix_web::main]
async fn main() {
    if let Err(error) = back::run_server().await {
        println!("{}", error);
        process::exit(1);
    }
}
