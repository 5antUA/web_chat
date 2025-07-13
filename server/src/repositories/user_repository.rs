use sqlx::{PgPool, Error as SqlxError};

use crate::models::user::User;

pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<User, SqlxError> {
    let query = "SELECT * FROM users WHERE id = $1";

    sqlx::query_as::<_, User>(query)
        .bind(id)
        .fetch_one(pool)
        .await
}
