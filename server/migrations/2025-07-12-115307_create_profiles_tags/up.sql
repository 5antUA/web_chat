-- Your SQL goes here

CREATE TABLE profiles_tags (
    fk_profile_id INTEGER REFERENCES profiles(id) ON DELETE CASCADE,
    fk_tag_id INTEGER REFERENCES tags(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT now(),
    CONSTRAINT profiles_tags_pk PRIMARY KEY (fk_profile_id, fk_tag_id)
)
