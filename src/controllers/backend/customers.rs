use actix_web::{ HttpResponse, Responder };
use askama::Template;

#[derive(Template)]
#[template(path="backend/customers/index.html")]
struct IndexTemplate<> {
    
}

pub async fn index() -> impl Responder {
    let index = IndexTemplate {};
    HttpResponse::Ok().body(index.render().unwrap())
}
