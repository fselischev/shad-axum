use std::sync::{Arc, Mutex};

use crate::User;

// https://docs.rs/axum/0.7.5/axum/extract/struct.State.html#when-states-need-to-implement-clone
// Your top level state type must implement Clone to be extractable with State.
#[derive(Clone)]
pub struct AppState {
    users: Arc<Mutex<Vec<User>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&self, u: User) {
        let mut data = self.users.lock().expect("poisoned");
        data.push(u)
    }
}
