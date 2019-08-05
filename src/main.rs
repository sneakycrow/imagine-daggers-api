extern crate dotenv;

use actix_web::client::Client;
use actix_web::{http, middleware, web, App, Error, HttpResponse, HttpServer, HttpRequest};
use actix_cors::Cors;
use futures::Future;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct SignupEmail {
  email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Contact {
  contacts: Vec<SignupEmail>
}

fn signup(item: web::Json<SignupEmail>) -> impl Future<Item = HttpResponse, Error = Error> {
  Client::new()
    .put("https://api.sendgrid.com/v3/marketing/contacts")
    .bearer_auth(env::var("SENDGRID_API_KEY").unwrap())
    .send_json(&Contact {
      contacts: vec![item.0]
    })
    .map_err(Error::from)
    .and_then(|mut response| {
      response
        .body()
        .from_err()
        .and_then(|body| {
          Ok(HttpResponse::Ok().body(body))
        })
    })
}

fn index(req: HttpRequest) -> &'static str {
  "Hello World"
}

fn main() {
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
      .service(web::resource("/signup").route(web::post().to_async(signup)))
      .service(web::resource("/").to(index))
  })
  .bind("127.0.0.1:8080")
  .unwrap()
  .run()
  .unwrap()
}