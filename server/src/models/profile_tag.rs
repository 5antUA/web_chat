use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow)]
pub struct ProfileTag {
    pub fk_profile_id: i32,
    pub fk_tag_id: i32,
    pub created_at: NaiveDateTime,
}
