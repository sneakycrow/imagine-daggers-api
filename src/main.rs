extern crate actix_web;

use actix_web::Result;

fn index() -> Result<String> {
  Ok(format!("Hello World!"))
}

fn main() {
  use actix_web::{web, App, HttpServer};

  HttpServer::new(|| App::new().route("/", web::get().to(index)))
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap()
}