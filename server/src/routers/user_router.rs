use crate::controllers::user_controller;

use actix_web::web;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/{username}", web::get().to(user_controller::get_user))
            .route("/register", web::post().to(user_controller::add_user))
            .route("/{username}", web::patch().to(user_controller::update_user)),
    );
}
