use crate::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    // cfg.service(
    //     web::resource("/customers/new")
    //         .route(web::get().to(handlers::backend::customers::new)),
    // )
    // .service(
    //     web::resource("/customers/{id}")
    //         .route(web::get().to(handlers::backend::customers::edit)),
    // )
    cfg.route("/customers", web::get().to(handlers::backend::customers::index))
       .route("/customers", web::post().to(handlers::backend::customers::create))
       .route("/customers/new", web::get().to(handlers::backend::customers::new)
            // .route(web::post().to(handlers::backend::customers::create)),
    );
}
