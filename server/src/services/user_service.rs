use sqlx::PgPool;

use crate::{
    models::user::{User, UserDTO},
    repositories::user_repository,
};

pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<User, String> {
    user_repository::get_user_by_id(pool, id)
        .await
        .map_err(|e| e.to_string())
}

pub async fn add_user(pool: &PgPool, user_dto: &UserDTO) -> Result<User, String> {
    user_repository::add_user(pool, user_dto)
        .await
        .map_err(|e| e.to_string())
}
