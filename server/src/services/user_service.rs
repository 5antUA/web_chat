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
    use sqlx::Error;

    match result {
        Ok(user) => Ok(user),
        Err(Error::RowNotFound) => Err(AppError::NotFound),
        Err(Error::Database(db_error)) => {
            let db_code = db_error.code().unwrap_or_default();

            match db_code.as_ref() {
                "23502" => Err(AppError::BadRequest), // спроба впихнути NULL
                "23503" => Err(AppError::BadRequest), // неіснуючий елемент
                "23505" => Err(AppError::Conflict),   // дублікат значення
                _ => Err(AppError::InternalServerError),
            }
        }
        _ => Err(AppError::InternalServerError),
    }
}
