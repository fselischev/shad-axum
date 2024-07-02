use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use shad_axum::{AppState, LogLayer, User};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState::new();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .layer(LogLayer::with_target("logger"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    State(state): State<AppState>,
    Query(_params): Query<HashMap<String, String>>,
    Json(payload): Json<User>,
) -> Response {
    let user = User::new(payload.id, payload.username);
    state.add(user.clone());
    (StatusCode::CREATED, Json(user)).into_response()
}
