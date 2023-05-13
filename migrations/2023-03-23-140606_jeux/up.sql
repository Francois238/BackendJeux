-- Your SQL goes here
CREATE TABLE snake (
  id SERIAL PRIMARY KEY,
  username TEXT UNIQUE NOT NULL,
  score INT NOT NULL
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT UNIQUE NOT NULL,
  password BYTEA NOT NULL
)