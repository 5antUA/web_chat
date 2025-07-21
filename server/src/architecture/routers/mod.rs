pub mod check_session;
pub mod profile_routers;
pub mod user_router;
// other routes

use actix_web::web::ServiceConfig;

pub fn configure_routes_all(cfg: &mut ServiceConfig) {
    check_session::check_session(cfg);
    user_router::user_routes(cfg);
    profile_routers::profile_routes(cfg);
}
