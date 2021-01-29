mod controllers;
mod routes;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(web::scope("/admin").configure(routes::backend::config))
            .service(web::scope("/").configure(routes::frontend::config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
