use sqlx::PgPool;

use crate::{models::user::User, repositories::user_repository};

pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<User, String> {
    user_repository::get_user_by_id(pool, id)
        .await
        .map_err(|e| e.to_string())
}
