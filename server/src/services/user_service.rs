use crate::{
    app_error::AppError,
    models::user::{User, UserDTO},
    repositories::user_repository,
};

use sqlx::PgPool;

pub async fn get_user(username: String, pool: &PgPool) -> Result<User, AppError> {
    match user_repository::get_user(username, pool).await {
        Ok(user) => Ok(user),
        Err(error) => match error {
            sqlx::Error::RowNotFound => Err(AppError::NotFound),
            _ => Err(AppError::InternalServerError),
        },
    }
}

pub async fn add_user(user: &UserDTO, pool: &PgPool) -> Result<User, String> {
    user_repository::add_user(user, pool)
        .await
        .map_err(|e| e.to_string())
}
