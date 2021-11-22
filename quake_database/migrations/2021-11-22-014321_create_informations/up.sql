-- Your SQL goes here

CREATE TABLE information (
   id SERIAL PRIMARY KEY,
   title VARCHAR NOT NULL,
   body TEXT NOT NULL,
   published BOOLEAN NOT NULL DEFAULT 'f'
)