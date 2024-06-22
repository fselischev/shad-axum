use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use crate::{AppState, LogLayer, User};

pub fn app() -> Router {
    let state = AppState::new();

    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .layer(LogLayer::with_target("logger"))
        .with_state(state)
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[debug_handler]
async fn create_user(
    Query(_params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> Response {
    let user = User::new(payload.id, payload.username);
    state.add(user.clone());
    (StatusCode::CREATED, Json(user)).into_response()
}
