use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow)]
pub struct Tag {
    pub id: i32,
    pub tag_name: String,
    pub created_at: NaiveDateTime,
}
