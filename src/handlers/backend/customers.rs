use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use askama::Template;
use crate::diesel::RunQueryDsl;
use diesel::dsl::insert_into;
use super::super::super::Pool;
use super::super::super::models::customer::*;
use super::super::super::schema::customers::dsl::*;


#[derive(Template)]
#[template(path = "backend/customers/index.html")]
struct IndexTemplate {}

pub async fn index() -> impl Responder {
    let index = IndexTemplate {};
    HttpResponse::Ok().body(index.render().unwrap())
}

#[derive(Template)]
#[template(path = "backend/customers/new.html")]
struct NewTemplate {}

pub async fn new() -> impl Responder {
    let new = NewTemplate {};
    HttpResponse::Ok().body(new.render().unwrap())
}

#[derive(Deserialize)]
pub struct FormData {
    first_name: String,
    last_name: String
}

pub async fn create(form: web::Form<FormData>, db: web::Data<Pool>) -> HttpResponse {
    log::info!("{} {}", form.first_name, form.last_name);
    let conn = db.get().expect("couldn't get db connection from pool");
    let new_customer = NewCustomer {
        first_name: &form.first_name,
        last_name: &form.last_name
    };
    insert_into(customers).values(&new_customer).execute(&conn); //.expect("Error saving new post");
    // log::info!("{}", res);
    HttpResponse::Found().header("Location", "/admin/customers").finish()
}

pub async fn edit() -> impl Responder {
    HttpResponse::Ok().body("")
}
