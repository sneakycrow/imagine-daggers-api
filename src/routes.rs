use actix_web::client::Client;
use futures::Future;
use std::env;
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupEmail {
  email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Contact {
  contacts: Vec<SignupEmail>
}


pub fn subscribe(item: web::Json<SignupEmail>) -> impl Future<Item = HttpResponse, Error = Error> {
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

pub fn index() -> &'static str {
  "Hello World, from Imagine Daggers API"
}