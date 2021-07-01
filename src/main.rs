// #[macro_use]
// extern crate diesel;

mod handlers;
mod routes;
// mod models;
// mod schema;

use actix_files::{NamedFile};
use actix_web::{web, App, HttpServer};
// use actix_web_static_files;
// use actix_web::middleware::Logger;
// use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
// use diesel::prelude::*;
// use diesel::r2d2::{self, ConnectionManager};

// pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_server=info,controllers=info,actix_web=info");
    env_logger::init();

    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let manager = ConnectionManager::<PgConnection>::new(database_url);
    // let pool: Pool = r2d2::Pool::builder()
    //     .build(manager)
    //     .expect("Failed to create pool;");
    
    HttpServer::new(move || {
        // let generated = generate();
        App::new()
            // .wrap(Logger::default())
            // .wrap(Logger::new("%a %{User-Agent}i"))
            // .data(pool.clone())
            // .service(actix_web_static_files::ResourceFiles::new(
            //     "/static", generated
            // ))
            // .service(web::scope("/admin").configure(routes::backend::config))
            .service(web::scope("/").configure(routes::frontend::config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
