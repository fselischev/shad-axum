use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tracing::{debug, error, info, instrument};

use crate::{User, UsersState};

#[instrument(skip(state))]
pub async fn create_user(
    State(state): State<UsersState>,
    Query(params): Query<HashMap<String, String>>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    let mut name = payload.name;
    if let Some(suffix) = params.get("suffix") {
        debug!("Found suffix: {}", suffix);
        name.push_str(suffix);
    }

    let user = User::new(name, payload.age);
    state.add(user.clone());
    info!(?user, "User created");
    (StatusCode::CREATED, Json(user))
}

#[instrument(skip(state))]
pub async fn get_user(
    State(state): State<UsersState>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let Some(id) = params.get("uuid") else {
        error!(?params, "Invalid request params");
        return StatusCode::NOT_FOUND.into_response();
    };

    match id.parse() {
        Ok(id) => {
            let Some(user) = state.get(id) else {
                error!(%id, "Cannot find user by uuid");
                return StatusCode::NOT_FOUND.into_response();
            };
            info!(?user, "User found");
            (StatusCode::FOUND, Json(user)).into_response()
        }
        Err(err) => {
            error!(%err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

#[instrument(skip(state))]
pub async fn update_user(
    State(state): State<UsersState>,
    Query(params): Query<HashMap<String, String>>,
    Json(payload): Json<User>,
) -> Response {
    let Some(id) = params.get("uuid") else {
        error!(?params, "Invalid request params");
        return StatusCode::NOT_FOUND.into_response();
    };

    match id.parse() {
        Ok(id) => {
            let Some(mut user) = state.get(id) else {
                error!(%id, "Cannot find user by uuid");
                return StatusCode::NOT_FOUND.into_response();
            };

            user.age = payload.age;
            user.name = payload.name;
            state.add(user.clone());

            info!(?user, "User updated");
            (StatusCode::FOUND, Json(user)).into_response()
        }
        Err(err) => {
            error!(%err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

#[instrument(skip(state))]
pub async fn delete_user(
    State(state): State<UsersState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let Some(id) = params.get("uuid") else {
        error!(?params, "Invalid request params");
        return StatusCode::NOT_FOUND.into_response();
    };

    match id.parse() {
        Ok(id) => {
            let user = state.get(id);
            state.remove(id);
            info!(?user, "User removed");
            (StatusCode::FOUND, Json(user)).into_response()
        }
        Err(err) => {
            error!(%err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}
