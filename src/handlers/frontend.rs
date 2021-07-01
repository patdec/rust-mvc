use std::collections::HashMap;

use actix_web::{ web, HttpResponse, Responder, Result };
use actix_files::{NamedFile};

pub async fn hello(tmpl: web::Data<tera::Tera>, query: web::Query<HashMap<String, String>>) -> impl Responder {
    let name = query.get("name").unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("name", &name.to_owned());
    let s = tmpl.render("frontend/hello.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}

pub async fn error500() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/error500.html")?)
}
