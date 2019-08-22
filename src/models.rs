use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupEmail {
  pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
  pub contacts: Vec<SignupEmail>
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
  pub id: String,
  pub username: String,
  pub password: String,
  pub email: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
  pub id: &'a str,
  pub username: &'a str,
  pub password: &'a str,
  pub email: &'a str
}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
  pub username: String,
  pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonUser {
  pub username: String,
  pub password: String,
  pub email: String
}