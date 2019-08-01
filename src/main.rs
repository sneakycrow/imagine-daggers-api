extern crate actix_web;
extern crate actix_files;
extern crate env_logger;

use actix_web::Result;
use actix_web::http::{StatusCode};
use actix_files::NamedFile;

fn index() -> Result<NamedFile> {
  Ok(NamedFile::open("dist/index.html")?.set_status_code(StatusCode::NOT_FOUND))
}

fn main() {
  use actix_web::{web, App, HttpServer};

  HttpServer::new(|| App::new().route("/", web::get().to(index)))
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap()
}