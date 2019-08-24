use actix_web::client::Client;
use futures::Future;
use std::env;
use actix_web::{web, Error, HttpResponse};
use crate::models;
use diesel::prelude::*;
use std::time::SystemTime;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;
use diesel::pg::PgConnection;
use bcrypt::{DEFAULT_COST, hash, verify};
use jwt::{encode, Header};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Subscribe the user to the maillist on sendgrid
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

// Create a new user
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

// Route for registering a new user
pub fn register(
    item: web::Json<models::JsonUser>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // run diesel blocking code
    web::block(move || create_user(item.into_inner(), pool)).then(|res| match res {
        Ok(user) => {
          let user_response = models::UserJWT {
            id: user.id,
            username: user.username,
            email: user.email,
            registration_date: user.registration_date,
            creation_timestamp: SystemTime::now()
          };
          let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable is not set");
          let encoded_user = encode(&Header::default(), &user_response, jwt_secret.as_ref());
          match encoded_user {
            Ok(jwt) => HttpResponse::Ok().json(jwt),
            Err(_) => HttpResponse::InternalServerError().into()
          }
        },
        Err(_) => HttpResponse::InternalServerError().into(),
    })
}

// Route for authenticating a user with a jwt token
pub fn login_user(
  item: web::Json<models::UserLogin>,
  pool: web::Data<Pool>
) -> HttpResponse {
  use crate::schema::users::dsl::*;

  let conn: &PgConnection = &pool.get().unwrap();
  let results: Result<models::User, _> = users.filter(username.eq(item.0.username)).first(conn);
  

  match results {
    Ok(results) => {
      let is_valid = verify(item.0.password, &results.password);
      if is_valid.unwrap() {
        let user_response = models::UserJWT {
          id: results.id,
          username: results.username,
          email: results.email,
          registration_date: results.registration_date,
          creation_timestamp: SystemTime::now()
        };
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable is not set");
        let encoded_user = encode(&Header::default(), &user_response, jwt_secret.as_ref());
        match encoded_user {
          Ok(jwt) => HttpResponse::Ok().json(jwt),
          Err(_) => HttpResponse::InternalServerError().into()
        }
      } else {
        HttpResponse::NotFound().into()
      }
    },
    Err(_) => HttpResponse::InternalServerError().into()
  }
}

// Route for listing all users
pub fn list_users(
  pool: web::Data<Pool>
) -> HttpResponse {
    use crate::schema::users::dsl::*;

    let conn: &PgConnection = &pool.get().unwrap();

    let user_list = users.load::<models::User>(conn);

    match user_list {
      Ok(user_list) => {
        let mut user_list_response = Vec::new();
        for user in user_list {
          let user_response = models::UserListSingle {
            email: user.email,
            username: user.username,
            registration_date: user.registration_date
          };
          user_list_response.push(user_response);
        };
        HttpResponse::Ok().json(user_list_response)
      },
      Err(_) => HttpResponse::InternalServerError().into()
    }
}

// Route for getting a specific user by email
pub fn get_user(path: web::Path<(String,)>, pool: web::Data<Pool>) -> HttpResponse {
  use crate::schema::users::dsl::*;

  let conn: &PgConnection = &pool.get().unwrap();

  let results: Result<models::User, _> = users.filter(username.eq(&path.0)).first(conn);

  match results {
    Ok(results) => {
      let user_response = models::UserJWT {
        id: results.id,
        username: results.username,
        email: results.email,
        registration_date: results.registration_date,
        creation_timestamp: SystemTime::now()
      };
      let encoded_user = encode(&Header::default(), &user_response, "secret".as_ref());
      match encoded_user {
        Ok(jwt) => HttpResponse::Ok().json(jwt),
        Err(_) => HttpResponse::InternalServerError().into()
      }
    },
    Err(_) => HttpResponse::InternalServerError().into()
  }
}

// Index route
pub fn index() -> &'static str {
  "Hello World, from Imagine Daggers API"
}