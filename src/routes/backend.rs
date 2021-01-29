use super::super::controllers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .service(web::scope("/customers")
                .route("/", web::get().to(controllers::backend::customers::index))
            )
    );
}
