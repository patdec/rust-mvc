use std::collections::HashMap;

use actix_web::{ error, web, Error, HttpResponse, Result };
use actix_files::{NamedFile};

pub async fn hello(tmpl: web::Data<tera::Tera>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
    let name = match query.get("name") {
        None => "",
        Some(n) => n
    };
    let mut ctx = tera::Context::new();
    ctx.insert("name", &name.to_owned());
    let s = tmpl.render("frontend/hello.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn error500() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/error500.html")?)
}
