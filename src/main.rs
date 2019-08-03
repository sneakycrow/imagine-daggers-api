extern crate dotenv;

use actix_web::client::Client;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
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

fn main() {
  dotenv().ok();
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .data(web::JsonConfig::default().limit(4096))
      .service(web::resource("/signup").route(web::post().to_async(signup)))
  })
  .bind("127.0.0.1:8080")
  .unwrap()
  .run()
  .unwrap()
}