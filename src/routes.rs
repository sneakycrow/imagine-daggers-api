use actix_web::client::Client;
use futures::Future;
use std::env;
use actix_web::{web, Error, HttpResponse};
use crate::models;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;
use diesel::pg::PgConnection;
use bcrypt::{DEFAULT_COST, hash};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn subscribe(item: web::Json<models::SignupEmail>) -> impl Future<Item = HttpResponse, Error = Error> {
  Client::new()
    .put("https://api.sendgrid.com/v3/marketing/contacts")
    .bearer_auth(env::var("SENDGRID_API_KEY").unwrap())
    .send_json(&models::Contact {
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

pub fn create_user<'a>(new_user: models::JsonUser, pool: web::Data<Pool>) -> Result<models::User, diesel::result::Error> {
  use crate::schema::users::dsl::*;

  let conn: &PgConnection = &pool.get().unwrap();
  
  let new_user_uuid = format!("{}", Uuid::new_v4());
  
  let hashed_password = hash(&new_user.password, DEFAULT_COST);

  let new_user_with_id = models::NewUser {
    id: &new_user_uuid,
    username: &new_user.username,
    password: &hashed_password.unwrap(),
    email: &new_user.email
  };

  diesel::insert_into(users)
    .values(&new_user_with_id)
    .execute(conn)?;

  let mut items = users.filter(id.eq(&new_user_uuid)).load::<models::User>(conn)?;
  Ok(items.pop().unwrap())
}

pub fn register(
    item: web::Json<models::JsonUser>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // run diesel blocking code
    web::block(move || create_user(item.into_inner(), pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn list_users(
  pool: web::Data<Pool>
) -> HttpResponse {
    use crate::schema::users::dsl::*;

    let conn: &PgConnection = &pool.get().unwrap();

    let results = users.limit(5).load::<models::User>(conn);

    match results {
      Ok(results) => HttpResponse::Ok().json(results),
      Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub fn index() -> &'static str {
  "Hello World, from Imagine Daggers API"
}