use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow)]
pub struct Profile {
    pub id: i32,
    pub fk_user_id: i32,
    pub age: i32,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
}
