use chrono::{Duration, NaiveDateTime, Utc};
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
pub struct UserInfo {
    pub username: String,
    pub password_hash: String,
    pub role_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserJWT {
    pub sub: Uuid, // Subject
    pub username: String,
    pub role_id: Uuid,
    pub iat: usize, // Issued At
    pub exp: usize, // Expiration Time
}

impl UserJWT {
    pub fn configure(user: &User) -> Self {
        let now = Utc::now();
        let expire = Duration::hours(24);

        Self {
            sub: user.id,
            username: user.username.clone(),
            role_id: user.fk_role_id,
            iat: now.timestamp() as usize,
            exp: (now + expire).timestamp() as usize,
        }
    }
}
