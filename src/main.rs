#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate bcrypt;

use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use bytes::BytesMut;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use futures::future::{err, Either};
use futures::{Future, Stream};
use bcrypt::{DEFAULT_COST, hash};

mod models;
mod schema;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// DIESEL QUERY
fn query(nm: String, pass: String, pool: web::Data<Pool>) -> Result<models::User, diesel::result::Error> {
  use self::schema::users::dsl::*;

  let uuid = format!("{}", uuid::Uuid::new_v4()); // UUID
  let hashed_password = hash(pass, DEFAULT_COST).expect("Failed to hash password");
  let new_user = models::NewUser {
    id: &uuid,
    name: nm.as_str(),
    password: &hashed_password
  };

  let conn: &PgConnection = &pool.get().unwrap();

  diesel::insert_into(users).values(&new_user).execute(conn)?;

  let mut items = users.filter(id.eq(&uuid)).load::<models::User>(conn)?;

  Ok(items.pop().unwrap())
}

#[derive(Debug, Serialize, Deserialize)]
struct MyUser {
  name: String,
  password: String
}

#[derive(Serialize)]
struct UserResponse {
  name: String,
  id: String
}

const MAX_SIZE: usize = 262_144; // Max payload size is 256l

// MANUALLY LOAD REQUEST PAYLOAD AND PARSE JSON OBJECT
fn index_add(
  pl: web::Payload,
  pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
  pl
    // `Future::from_err` acts like a `?` in that it coerces the error type from the future into the final error type
    .from_err()
    // `fold` will asynchronously read each chunk of the request body and call supplied closure, then it resolves to the result of the close
    .fold(BytesMut::new(), move |mut body, chunk| {
      // limit max size of in-memory payload
      if (body.len() + chunk.len()) > MAX_SIZE {
        Err(error::ErrorBadRequest("overflow"))
      } else {
        body.extend_from_slice(&chunk);
        Ok(body)
      }
    })
    .and_then(move |body| {
      // body is loaded, now we can deserialize serde-json
      let r_obj = serde_json::from_slice::<MyUser>(&body);

      // Send to the db for create
      match r_obj {
        Ok(obj) => Either::A(
          web::block(move || query(obj.name, obj.password, pool)).then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(UserResponse { name: user.name, id: user.id})),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
          }),
        ),
        Err(_) => Either::B(err(error::ErrorBadRequest("JSON DECODE FAILED"))),
      }
    })
}

fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  dotenv::dotenv().ok();

  let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
  let manager = ConnectionManager::<PgConnection>::new(connspec);
  let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool");

  // START HTTP SERVER
  HttpServer::new(move || {
    App::new()
    .data(pool.clone())
    // enable logger
    .wrap(middleware::Logger::default())
    .service(web::resource("/add").route(web::post().to_async(index_add)))
  })
  .bind("127.0.0.1:8081")?
  .run()
}