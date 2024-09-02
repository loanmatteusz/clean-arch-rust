use actix_web::web;
use crate::presentation::handlers::user_handler::{get_user_handler, register_user_handler};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/users")
            .service(register_user_handler)
            .service(get_user_handler)
    );
}
