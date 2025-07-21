use actix_web::{
    middleware,
    web::{self, ServiceConfig},
};

use crate::architecture::middlewares::auth::verify_jwt;

pub fn check_session(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/check_session").wrap(middleware::from_fn(verify_jwt)));
}
