use shad_axum::app;
use tokio::signal;
use tracing::{info, instrument};

#[instrument]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let sigint = async {
        signal::ctrl_c().await.unwrap();
    };

    #[cfg(unix)]
    let sigterm = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = sigint => {},
        _ = sigterm => {},
    }
}
