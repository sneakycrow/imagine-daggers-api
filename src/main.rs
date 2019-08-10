#[macro_use]
extern crate diesel;

mod schema;
mod routes;
mod models;

use actix_web::{http, middleware, web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use routes::{subscribe, index, register};

fn main() -> std::io::Result<()> {
  dotenv().ok();
  openssl_probe::init_ssl_cert_env_vars();
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
  let manager = ConnectionManager::<PgConnection>::new(connspec);
  let pool = r2d2::Pool::builder()
      .build(manager)
      .expect("Failed to create pool.");

  HttpServer::new(move || {
    App::new()
      .data(pool.clone())
      .wrap(middleware::Logger::default())
      .wrap(
            Cors::new()
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600))
      .data(web::JsonConfig::default().limit(4096))
      .service(web::resource("/signup").route(web::post().to_async(subscribe)))
      .service(web::resource("/register").route(web::post().to_async(register)))
      .service(web::resource("/").to(index))
  })
  .bind("0.0.0.0:8080")?
  .run()
}