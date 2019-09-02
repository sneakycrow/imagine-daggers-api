extern crate openssl;
#[macro_use]
extern crate diesel;
extern crate jsonwebtoken as jwt;

mod schema;
mod routes;
mod models;

use actix_web::{http, middleware, web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use routes::{subscribe, index, register, list_users, get_user_by_username, login_user, delete_user_by_id};

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

  let listen_port = std::env::var("PORT").expect("PORT var needs to be defined in environment");
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
      .service(web::resource("/subscribe").route(web::post().to_async(subscribe)))
      .service(web::resource("/register").route(web::post().to_async(register)))
      .service(web::resource("/users").route(web::get().to(list_users)))
      .service(web::resource("/users/{username}").route(web::get().to(get_user_by_username)))
      .service(web::resource("/login").route(web::post().to(login_user)))
      .service(web::resource("/delete/{user_id}").route(web::delete().to(delete_user_by_id)))
      .service(web::resource("/").to(index))
  })
  .bind(format!("0.0.0.0:{}", listen_port))?
  .run()
}