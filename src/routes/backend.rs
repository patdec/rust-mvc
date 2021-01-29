use super::super::controllers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/customers/new")
            .route(web::get().to(controllers::backend::customers::new)),
    )
    .service(
        web::resource("/customers/{id}")
            .route(web::get().to(controllers::backend::customers::edit)),
    )
    .service(
        web::resource("/customers")
            .route(web::get().to(controllers::backend::customers::index))
            .route(web::post().to(controllers::backend::customers::create)),
    );
}
