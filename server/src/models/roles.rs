use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub role_name: String,
    pub created_at: NaiveDateTime,
}
