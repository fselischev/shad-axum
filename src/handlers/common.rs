use axum::{http::StatusCode, response::IntoResponse};
use tracing::{instrument, trace};

#[instrument]
pub async fn heartbeat() -> impl IntoResponse {
    trace!("OK");
    "heartbeat"
}

#[instrument]
pub async fn fallback() -> impl IntoResponse {
    trace!("404: not found");
    (StatusCode::NOT_FOUND, "Not Found :(")
}
