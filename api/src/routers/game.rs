use axum::{
    extract::{Path, State},
    routing::{get, patch, post},
    Json, Router,
};

use crate::{AppState, Game, User};

async fn get_games(State(state): State<AppState>, _: User) -> Json<Vec<Game>> {
    todo!();
}

async fn get_game(State(state): State<AppState>, _: User, Path(id): Path<i32>) -> Json<Game> {
    todo!();
}

async fn post_game(State(state): State<AppState>, _: User, game: Json<Game>) -> Json<Game> {
    todo!();
}

async fn patch_game(State(state): State<AppState>, _: User) -> Json<Game> {
    todo!();
}

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/games", get(get_games))
        .route("/game", get(get_game))
        .route("/game", post(post_game))
        .route("/game", patch(patch_game))
}
