use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(skip_deserializing)]
    pub uuid: u128,
    pub name: String,
    pub age: u8,
}

impl User {
    pub fn new(name: String, age: u8) -> Self {
        Self {
            uuid: Uuid::new_v4().as_u128(),
            name,
            age,
        }
    }
}
