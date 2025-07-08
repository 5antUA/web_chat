use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    username: String,
    id: i32,
}

impl User {}
