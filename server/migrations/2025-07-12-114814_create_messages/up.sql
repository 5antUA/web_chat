-- Your SQL goes here

CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    fk_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    message_content TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT now()
)
