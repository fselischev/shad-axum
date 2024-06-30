use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_prometheus::PrometheusMetricLayer;
use tracing::{error, info, instrument, Level};

use crate::{AppState, LogLayer, User};

pub fn app() -> Router {
    let state = AppState::new();
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .layer(LogLayer::with_target("app"))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .with_state(state)
}

#[instrument]
async fn root(Query(params): Query<HashMap<String, String>>) -> String {
    // can think of it like a expanding of instument macro but compatible in async context compatible
    // let span = tracing::span!(Level::TRACE, "root");
    // let _guard = span.enter();

    match params.get("name") {
        Some(name) => {
            info!("done with name: {}", name);
            format!("Hello, {}!", name)
        }
        None => {
            error!("error with no name provided");
            String::from("Please provide name as query parameter")
        }
    }
}

#[debug_handler]
async fn create_user(
    Query(_params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> Result<Response, UserError> {
    if payload.id > 10_000 {
        return Err(panic().err().unwrap().into());
    }

    let user = User::new(payload.id, payload.username);
    state.add(user.clone());
    Ok((StatusCode::CREATED, Json(user)).into_response())
}

struct UserError(anyhow::Error);
impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0),
        )
            .into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for UserError {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

fn panic() -> Result<(), anyhow::Error> {
    anyhow::bail!("Oops, maximum users exceeded..\n")
}
