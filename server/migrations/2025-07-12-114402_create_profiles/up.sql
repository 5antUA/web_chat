-- Your SQL goes here

CREATE TABLE profiles (
    id SERIAL PRIMARY KEY,
    fk_user_id INTEGER UNIQUE NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    age INTEGER NOT NULL,
    bio TEXT,
    avatar_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);
