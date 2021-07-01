use super::super::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/error500", web::get().to(handlers::frontend::error500))
        .route("/hello", web::get().to(handlers::frontend::hello));
}
