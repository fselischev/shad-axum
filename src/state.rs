use std::sync::{Arc, Mutex};

use crate::User;

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
