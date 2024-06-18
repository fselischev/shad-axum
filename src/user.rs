use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Clone, Serialize)]
pub struct User {
    id: u64,
    username: String,
}

impl User {
    pub fn new(id: u64, username: String) -> Self {
        Self { id, username }
    }
}
