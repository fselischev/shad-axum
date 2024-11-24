use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::User;

// https://docs.rs/axum/0.7.5/axum/extract/struct.State.html#when-states-need-to-implement-clone
// Your top level state type must implement Clone to be extractable with State.
#[derive(Debug, Clone)]
pub struct UsersState {
    users: Arc<Mutex<HashMap<u128, User>>>,
}

impl UsersState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add(&self, u: User) {
        let mut map = self.users.lock().expect("poisoned");
        map.insert(u.uuid, u);
    }

    pub fn get(&self, uuid: u128) -> Option<User> {
        let map = self.users.lock().expect("poisoned");
        map.get(&uuid).cloned()
    }

    pub fn remove(&self, uuid: u128) -> Option<User> {
        let mut map = self.users.lock().expect("poisoned");
        map.remove(&uuid)
    }
}
