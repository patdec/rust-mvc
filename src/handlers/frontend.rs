use actix_web::{ HttpResponse, Responder, Result };
use askama::Template;
use actix_files::NamedFile;

#[derive(Template)]
#[template(path="frontend/hello.html")]
struct HelloTemplate<'a> {
    name: &'a str
}

pub async fn hello() -> impl Responder {
    let hello = HelloTemplate { name: "Patrice" };
    HttpResponse::Ok().body(hello.render().unwrap())
}

pub async fn error500() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/error500.html")?)
}
