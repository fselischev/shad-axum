use axum::{http::StatusCode, response::IntoResponse};
use tokio::signal;
use tracing::{debug, instrument, trace};

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

#[instrument]
pub async fn shutdown_signal() {
    let sigint = async {
        signal::ctrl_c()
            .await
            .expect("failed to install signal handler");
    };

    #[cfg(unix)]
    let sigterm = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = sigint => {
            debug!("sigint handled")
        },
        _ = sigterm => {
            debug!("sigterm handled")
        },
    }
}
