use crate::controllers::user_controller;

use actix_web::web;

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/{username}", web::get().to(user_controller::get_user_by_username))
            .route("/register", web::get().to(user_controller::add_user)),
    );
}
