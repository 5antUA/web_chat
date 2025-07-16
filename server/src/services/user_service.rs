use crate::{
    app_error::AppError,
    models::user::{User, UserDTO},
    repositories::user_repository,
};

use bcrypt::DEFAULT_COST;
use sqlx::PgPool;

// get user
pub async fn get_user(username: String, pool: &PgPool) -> Result<User, AppError> {
    let sql_result = user_repository::get_user(&username, pool).await;
    sql_result.map_err(AppError::from)
}

// add or register user
pub async fn add_user(user: &mut UserDTO, pool: &PgPool) -> Result<User, AppError> {
    let hashed_password = match bcrypt::hash(&user.password_hash, DEFAULT_COST) {
        Ok(pass) => pass,
        Err(_) => return Err(AppError::InternalServerError),
    };

    user.password_hash = hashed_password;

    let sql_result = user_repository::add_user(&user, pool).await;
    sql_result.map_err(AppError::from)
}

// login user
pub async fn login_user(user: &UserDTO, pool: &PgPool) -> Result<bool, AppError> {
    let sql_result = user_repository::get_user(&user.username, pool).await;

    match sql_result.map_err(AppError::from) {
        Ok(sql_user) => {
            let login_condition = bcrypt::verify(&user.password_hash, &sql_user.password_hash)
                .map_err(|_| AppError::Unauthorized)?;

            Ok(login_condition)
        }
        Err(error) => Err(error),
    }
}
