use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use shad_axum::{AuthChecker, AuthLayer, User};
use tracing::{debug, info, instrument, trace, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = Router::new()
        .route("/heartbeat", get(heartbeat))
        .route(
            "/user",
            post(create_user)
                .get(get_user)
                .put(update_user)
                .delete(delete_user),
        )
        .fallback(fallback)
        .layer(AuthLayer::new());

    let listener = tokio::net::TcpListener::bind("[::1]:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[instrument]
async fn fallback() -> impl IntoResponse {
    trace!("404: not found");
    (StatusCode::NOT_FOUND, "Not Found :(")
}

#[instrument]
async fn heartbeat() -> impl IntoResponse {
    trace!("OK");
    "heartbeat"
}

#[instrument]
async fn create_user(
    Query(params): Query<HashMap<String, String>>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    let mut name = payload.username;
    if let Some(suffix) = params.get("suffix") {
        trace!("Found suffix: {}", suffix);
        name.push_str(suffix);
    }

    let user = User::new(name, payload.age);
    debug!(?user, "User created");
    (StatusCode::CREATED, Json(user))
}

#[instrument]
async fn get_user(
    Query(_params): Query<HashMap<String, String>>,
    Json(_payload): Json<User>,
) -> impl IntoResponse {
    unimplemented!("no storage to extract from")
}

#[instrument]
async fn update_user(
    Query(_params): Query<HashMap<String, String>>,
    Json(_payload): Json<User>,
) -> impl IntoResponse {
    unimplemented!("no storage to extract from")
}

#[instrument]
async fn delete_user(
    Query(_params): Query<HashMap<String, String>>,
    Json(_payload): Json<User>,
) -> impl IntoResponse {
    unimplemented!("no storage to extract from")
}
