use crate::controllers::user_controller;

use actix_web::web;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/{username}", web::get().to(user_controller::get_user))
            .route("/register", web::post().to(user_controller::register_user))
            .route("/login", web::post().to(user_controller::login_user)),
    );
}
