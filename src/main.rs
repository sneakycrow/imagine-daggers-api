mod routes;

use actix_web::{http, middleware, web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use routes::{subscribe, index};

fn main() -> std::io::Result<()> {
  dotenv().ok();
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .wrap(
            Cors::new()
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600))
      .data(web::JsonConfig::default().limit(4096))
      .service(web::resource("/signup").route(web::post().to_async(subscribe)))
      .service(web::resource("/").to(index))
  })
  .bind("127.0.0.1:8080")?
  .run()
}