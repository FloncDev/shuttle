use axum::{
    extract::{Path, State},
    routing::{get, patch, post},
    Json, Router,
};

use crate::{AppState, DuoGame, Result, User};

async fn get_games(State(state): State<AppState>) -> Result<Json<Vec<DuoGame>>> {
    let games_db = sqlx::query!(
        r#"
            SELECT
            duo_games.id, duo_games.created_at, duo_games.first_to,

            t1.id as team_one_id,
            t1p1.id as team_one_player_one_id,
            t1p1.name as team_one_player_one_name,
            t1p2.id as team_one_pl1ayer_two_id,
            t1p2.name as team_one_player_two_name,

            t2.id as team_two_id,
            t2p1.id as team_two_player_one_id,
            t2p1.name as team_two_player_one_name,
            t2p2.id as team_two_pl1ayer_two_id,
            t2p2.name as team_two_player_two_name
            FROM duo_games
            JOIN teams t1 ON t1.id = duo_games.team_one
            JOIN users t1p1 ON t1.player_one = t1p1.id
            LEFT JOIN users t1p2 ON t1.player_two = t1p2.id

            JOIN teams t2 ON t2.id = duo_games.team_two
            JOIN users t2p1 ON t2.player_one = t2p1.id
            LEFT JOIN users t2p2 ON t2.player_two = t2p2.id
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    todo!();
}

async fn get_game(State(state): State<AppState>, _: User, Path(id): Path<i32>) -> Json<DuoGame> {
    todo!();
}

async fn post_game(State(state): State<AppState>, _: User, game: Json<DuoGame>) -> Json<DuoGame> {
    todo!();
}

async fn patch_game(State(state): State<AppState>, _: User) -> Json<DuoGame> {
    todo!();
}

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/duo/games", get(get_games))
        .route("/duo/game/:id", get(get_game))
        .route("/duo/game", post(post_game))
        .route("/duo/game", patch(patch_game))
}
