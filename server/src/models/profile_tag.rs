use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ProfileTag {
    pub fk_profile_id: i32,
    pub fk_tag_id: i32,
    pub created_at: NaiveDateTime,
}
