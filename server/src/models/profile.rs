use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Profile {
    pub id: Uuid,
    pub fk_user_id: Uuid,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
}
