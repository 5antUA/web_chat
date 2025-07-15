use crate::{
    app_error::AppError,
    models::user::{User, UserDTO},
    repositories::user_repository,
};

use sqlx::PgPool;

pub async fn get_user(username: String, pool: &PgPool) -> Result<User, AppError> {
    match_responce(user_repository::get_user(username, pool).await)
}

pub async fn add_user(user: &UserDTO, pool: &PgPool) -> Result<User, AppError> {
    match_responce(user_repository::add_user(user, pool).await)
}

pub async fn update_user(user: &UserDTO, pool: &PgPool) -> Result<User, AppError> {
    match_responce(user_repository::update_user(user, pool).await)
}

fn match_responce<T>(result: Result<T, sqlx::Error>) -> Result<T, AppError> {
    match result {
        Ok(user) => Ok(user),
        Err(error) => Err(match error {
            sqlx::Error::RowNotFound => AppError::NotFound,
            sqlx::Error::Database(_) => AppError::Conflict,
            _ => AppError::InternalServerError,
        }),
    }
}
