use super::super::controllers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("/hello",web::get().to(controllers::frontend::hello))
    );
}
