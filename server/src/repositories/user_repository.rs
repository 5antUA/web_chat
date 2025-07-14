use crate::models::user::{User, UserDTO};

use sqlx::{Error as SqlxError, PgPool};

pub async fn get_user_by_id(username: String, pool: &PgPool) -> Result<User, SqlxError> {
    let getted_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users 
        WHERE username = $1",
    )
    .bind(username)
    .fetch_one(pool)
    .await?;

    Ok(getted_user)
}

pub async fn add_user(user: &UserDTO, pool: &PgPool) -> Result<User, SqlxError> {
    let received_user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash, fk_role_id) 
        VALUES ($1, $2, (SELECT id FROM roles WHERE role_name = $3))
        RETURNING *",
    )
    .bind(&user.username)
    .bind(&user.password_hash)
    .bind(&user.role_name)
    .fetch_one(pool)
    .await?;

    Ok(received_user)
}
