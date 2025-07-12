use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    id: i32,
}

pub struct AppData {
    pub app_name: String,
    pub author_name: String,
    pub version: i16,
}

impl User {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}
