use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    // id: i32,
}

impl User {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    // pub fn get_id(&self) -> i32 {
    //     self.id
    // }
}
