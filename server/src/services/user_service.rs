use crate::{
    models::user::{User, UserDTO},
    repositories::user_repository,
};

use sqlx::PgPool;

pub async fn get_user_by_id(username: String, pool: &PgPool) -> Result<User, String> {
    user_repository::get_user_by_id(username, pool)
        .await
        .map_err(|e| e.to_string())
}

pub async fn add_user(user: &UserDTO, pool: &PgPool) -> Result<User, String> {
    user_repository::add_user(user, pool)
        .await
        .map_err(|e| e.to_string())
}
