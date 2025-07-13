use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Message {
    pub id: i32,
    pub fk_user_id: i32,
    pub message_content: Option<String>,
    pub created_at: NaiveDateTime,
}
