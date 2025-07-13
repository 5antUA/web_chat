use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Tag {
    pub id: i32,
    pub tag_name: String,
    pub created_at: NaiveDateTime,
}
