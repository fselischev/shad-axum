use shad_axum::app;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}
