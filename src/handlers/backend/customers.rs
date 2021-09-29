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
use log::{info};


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn index(tmpl: web::Data<tera::Tera>, pool: web::Data<DbPool> ) -> Result<HttpResponse, Error> {
    let connection = pool.get()
        .expect("Can't get db connection from pool");

    let customers_data = web::block(move || {
        customers.limit(100).load::<Customer>(&connection)
    })
        .await
        .map_err(|_| {
            info!("ERR_CUSTOMERS");
            HttpResponse::InternalServerError().finish()
        })?;

    info!("CUSTOMERS: {:#?}", customers_data);
    let mut ctx = tera::Context::new();
    ctx.insert("customers", &customers_data);
    let s = tmpl.render("backend/customers/index.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(s))
}

pub async fn new(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let ctx = tera::Context::new();
    let s = tmpl.render("backend/customers/new.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(s))
}

pub async fn create(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<NewCustomer>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let connection = pool.get()
        .expect("Can't get db connection from pool");

    let customer = NewCustomer {
        first_name: params.first_name.to_string(),
        last_name: params.last_name.to_string()
    };

    web::block(move ||
               diesel::insert_into(customers)
               .values(&customer)
               .execute(&connection)
    )
        .await
        .map_err(|_| HttpResponse::InternalServerError())?;
    
    let ctx = tera::Context::new();
    let s = tmpl.render("backend/customers/new.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(s))
}

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
