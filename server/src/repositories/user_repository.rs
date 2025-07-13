use sqlx::{Error as SqlxError, PgPool};

use crate::models::user::{User, UserDTO};

pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<User, SqlxError> {
    let query = "SELECT * FROM users WHERE id = $1";

    sqlx::query_as::<_, User>(query)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn add_user(pool: &PgPool, user_dto: &UserDTO) -> Result<User, SqlxError> {
    let query =
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *";

    sqlx::query_as::<_, User>(query)
        .bind(&user_dto.username)
        .bind(&user_dto.email)
        .bind(&user_dto.password_hash)
        .fetch_one(pool)
        .await
}
