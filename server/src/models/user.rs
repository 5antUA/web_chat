use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::{types::Uuid, FromRow};

#[derive(Debug, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}
