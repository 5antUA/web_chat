use actix_web::{middleware, web};

use crate::architecture::{controllers::profile_controller, middlewares::auth};

pub fn profile_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profiles").service(
            web::resource("/nigger")
                .wrap(middleware::from_fn(auth::verify_jwt))
                .route(web::get().to(profile_controller::test)),
        ),
    );
}
