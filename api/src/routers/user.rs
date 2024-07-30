use axum::{extract::State, routing::get, Json, Router};

use crate::{AppState, User};

async fn get_me(State(_): State<AppState>, user: User) -> Json<User> {
    Json(user)
}

pub fn get_router() -> Router<AppState> {
    Router::new().route("/me", get(get_me))
}
