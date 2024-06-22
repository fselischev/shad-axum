use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}

impl User {
    pub fn new(id: u64, username: String) -> Self {
        Self { id, username }
    }
}
