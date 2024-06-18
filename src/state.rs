use crate::User;

#[derive(Clone)]
pub struct AppState {
    users: Vec<User>,
}

impl AppState {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    pub fn add(&mut self, u: User) {
        self.users.push(u);
    }
}
