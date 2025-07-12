-- Your SQL goes here

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    tag_name VARCHAR(20) UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
)
