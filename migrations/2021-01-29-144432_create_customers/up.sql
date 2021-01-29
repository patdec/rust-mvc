-- Your SQL goes here
CREATE TABLE customers (
       id SERIAL PRIMARY KEY,
       first_name VARCHAR NOT NULL,
       last_name VARCHAR NOT NULL,
       active BOOLEAN NOT NULL DEFAULT 't'
)
