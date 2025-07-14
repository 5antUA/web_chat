use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub fk_user_id: Uuid,
    pub message_content: Option<String>,
    pub created_at: NaiveDateTime,
}
