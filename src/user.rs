use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: u128,
    pub username: String,
    pub age: u8,
}

impl User {
    pub fn new(username: String, age: u8) -> Self {
        Self {
            id: Uuid::new_v4().as_u128(),
            username,
            age,
        }
    }
}
