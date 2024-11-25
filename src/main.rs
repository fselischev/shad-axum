use axum::{
    routing::{get, post},
    Router,
};
use shad_axum::{
    create_user, delete_user, fallback, get_user, heartbeat, shutdown_signal, update_user,
    AuthLayer, UsersState,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

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
        .layer(AuthLayer::new())
        .with_state(UsersState::new());

    let listener = tokio::net::TcpListener::bind("[::1]:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Serving failed");
}
