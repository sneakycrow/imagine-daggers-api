-- Your SQL goes here
CREATE TABLE users (
  id VARCHAR PRIMARY KEY,
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  email VARCHAR NOT NULL
)