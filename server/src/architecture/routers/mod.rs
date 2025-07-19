pub mod user_router;
// other routes

use actix_web::web::ServiceConfig;

pub fn configure_routes_all(cfg: &mut ServiceConfig) {
    user_router::user_routes(cfg);
}
