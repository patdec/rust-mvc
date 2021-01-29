use super::super::controllers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/hello")
            .route(web::get().to(controllers::frontend::hello))
    );
}
