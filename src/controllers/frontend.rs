use actix_web::{ HttpResponse, Responder };
use askama::Template;

#[derive(Template)]
#[template(path="frontend/hello.html")]
struct HelloTemplate<'a> {
    name: &'a str
}

pub async fn hello() -> impl Responder {
    let hello = HelloTemplate { name: "Patrice" };
    HttpResponse::Ok().body(hello.render().unwrap())
}
