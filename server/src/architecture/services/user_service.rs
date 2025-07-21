use std::sync::Arc;

use crate::{
    AppData,
    app_error::AppError,
    models::user::{User, UserInfo, UserJWT},
    repositories::user_repository,
    utils::jwt,
};

use bcrypt::DEFAULT_COST;

// get user
pub async fn get_user(username: String, app_data: &Arc<AppData>) -> Result<User, AppError> {
    let sql_result = user_repository::get_user(&username, &app_data.pool).await;
    sql_result.map_err(AppError::from)
}

// register user
pub async fn register_user(user: &mut UserInfo, app_data: &Arc<AppData>) -> Result<User, AppError> {
    let hashed_password = bcrypt::hash(&user.password_hash, DEFAULT_COST)
        .map_err(|_| AppError::InternalServerError)?;

    user.password_hash = hashed_password;

    let sql_result = user_repository::add_user(&user, &app_data.pool).await;
    sql_result.map_err(AppError::from)
}

// login user
pub async fn login_user(user: &UserInfo, app_data: &Arc<AppData>) -> Result<String, AppError> {
    let sql_result = user_repository::get_user(&user.username, &app_data.pool).await;

    let received_user = sql_result.map_err(AppError::from)?;
    let is_verify = bcrypt::verify(&user.password_hash, &received_user.password_hash)
        .map_err(|_| AppError::Unauthorized)?;

    if is_verify {
        let role_name =
            user_repository::get_role_name(&received_user.fk_role_id, &app_data.pool).await?;

        let claims = UserJWT::configure(&received_user, role_name);
        let token = jwt::encode_jwt(&claims, &app_data.jwt_secret)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        return Ok(token);
    }

    Err(AppError::Unauthorized)
}
