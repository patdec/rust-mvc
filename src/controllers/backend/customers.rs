use actix_web::{HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "backend/customers/index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "backend/customers/new.html")]
struct NewTemplate {}

pub async fn index() -> impl Responder {
    let index = IndexTemplate {};
    HttpResponse::Ok().body(index.render().unwrap())
}

pub async fn new() -> impl Responder {
    let new = NewTemplate {};
    HttpResponse::Ok().body(new.render().unwrap())
}

pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("")
}

pub async fn edit() -> impl Responder {
    HttpResponse::Ok().body("")
}
