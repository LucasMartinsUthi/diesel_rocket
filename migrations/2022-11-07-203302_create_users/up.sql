-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  location VARCHAR NOT NULL,
  title VARCHAR NOT NULL
)