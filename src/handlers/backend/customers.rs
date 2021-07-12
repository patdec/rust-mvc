use actix_web::{web, HttpResponse, Error};
// use serde::Deserialize;
// use crate::diesel::RunQueryDsl;
// use diesel::dsl::insert_into;
// use super::super::super::Pool;
// use super::super::super::models::customer::*;
use crate::models::customer::*;
use crate::schema::customers::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn index(tmpl: web::Data<tera::Tera>, pool: web::Data<DbPool> ) -> Result<HttpResponse, Error> {
    let connection = pool.get()
        .expect("Can't get db connection from pool");

    let customers_data = web::block(move || {
        customers.limit(100).load::<Customer>(&connection)
    })
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let mut ctx = tera::Context::new();
    ctx.insert("customers", &customers_data);
    let s = tmpl.render("frontend/hello.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(s))
}

// #[derive(Template)]
// #[template(path = "backend/customers/new.html")]
// struct NewTemplate {}

// pub async fn new() -> impl Responder {
//     let new = NewTemplate {};
//     HttpResponse::Ok().body(new.render().unwrap())
// }

// #[derive(Deserialize)]
// pub struct FormData {
//     first_name: String,
//     last_name: String
// }
//
// pub async fn create(form: web::Form<FormData>, db: web::Data<Pool>) -> HttpResponse {
//     log::info!("{} {}", form.first_name, form.last_name);
//     let conn = db.get().expect("couldn't get db connection from pool");
//     let new_customer = NewCustomer {
//         first_name: &form.first_name,
//         last_name: &form.last_name
//     };
//     insert_into(customers).values(&new_customer).execute(&conn); //.expect("Error saving new post");
//     // log::info!("{}", res);
//     HttpResponse::Found().header("Location", "/admin/customers").finish()
// }
//
// pub async fn edit() -> impl Responder {
//     HttpResponse::Ok().body("")
// }
