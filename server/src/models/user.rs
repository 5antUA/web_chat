use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub fk_role_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDTO {
    pub username: String,
    pub password_hash: String,
    pub role_name: String,
}
