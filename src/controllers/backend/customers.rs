use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use askama::Template;

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

pub async fn create(form: web::Form<FormData>) -> impl Responder {
    log::info!("{} {}", form.first_name, form.last_name);
    HttpResponse::Ok().body(format!("{} {}", form.first_name, form.last_name))
}

pub async fn edit() -> impl Responder {
    HttpResponse::Ok().body("")
}
