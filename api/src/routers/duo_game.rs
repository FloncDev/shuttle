use axum::{
    extract::{Path, State},
    routing::{get, patch, post},
    Json, Router,
};

use crate::{AppState, DuoGame, Result, User};

async fn get_games(State(state): State<AppState>) -> Result<Json<Vec<DuoGame>>> {
    Ok(Json(
        sqlx::query_as(
            r#"
            SELECT
            duo_games.id, duo_games.created_at, duo_games.first_to,

            t1.id as t1_id,
            t1p1.id as t1p1_id,
            t1p1.name as t1p1_name,
            t1p1.rating as t1p1_rating,
            t1p1.uncertainty as t1p1_uncertainty,

            t1p2.id as t1p2_id,
            t1p2.name as t1p2_name,
            t1p2.rating as t1p2_rating,
            t1p2.uncertainty as t1p2_uncertainty,

            t2.id as t2_id,
            t2p1.id as t2p1_id,
            t2p1.name as t2p1_name,
            t2p1.rating as t2p1_rating,
            t2p1.uncertainty as t2p1_uncertainty,

            t2p2.id as t2p2_id,
            t2p2.name as t2p2_name,
            t2p2.rating as t2p2_rating,
            t2p2.uncertainty as t2p2_uncertainty

            FROM duo_games
            JOIN teams t1 ON t1.id = duo_games.team_one
            JOIN players t1p1 ON t1.player_one = t1p1.id
            JOIN players t1p2 ON t1.player_two = t1p2.id

            JOIN teams t2 ON t2.id = duo_games.team_two
            JOIN players t2p1 ON t2.player_one = t2p1.id
            JOIN players t2p2 ON t2.player_two = t2p2.id
        "#,
        )
        .fetch_all(&state.pool)
        .await?,
    ))
}

async fn get_game(
    State(state): State<AppState>,
    _: User,
    Path(id): Path<i32>,
) -> Result<Json<DuoGame>> {
    Ok(Json(
        sqlx::query_as(
            r#"
            SELECT
            duo_games.id, duo_games.created_at, duo_games.first_to,

            t1.id as t1_id,
            t1p1.id as t1p1_id,
            t1p1.name as t1p1_name,
            t1p1.rating as t1p1_rating,
            t1p1.uncertainty as t1p1_uncertainty,

            t1p2.id as t1p2_id,
            t1p2.name as t1p2_name,
            t1p2.rating as t1p2_rating,
            t1p2.uncertainty as t1p2_uncertainty,

            t2.id as t2_id,
            t2p1.id as t2p1_id,
            t2p1.name as t2p1_name,
            t2p1.rating as t2p1_rating,
            t2p1.uncertainty as t2p1_uncertainty,

            t2p2.id as t2p2_id,
            t2p2.name as t2p2_name,
            t2p2.rating as t2p2_rating,
            t2p2.uncertainty as t2p2_uncertainty

            FROM duo_games
            JOIN teams t1 ON t1.id = duo_games.team_one
            JOIN users t1p1 ON t1.player_one = t1p1.id
            LEFT JOIN users t1p2 ON t1.player_two = t1p2.id

            JOIN teams t2 ON t2.id = duo_games.team_two
            JOIN users t2p1 ON t2.player_one = t2p1.id
            LEFT JOIN users t2p2 ON t2.player_two = t2p2.id
            WHERE duo_game.id = $1
        "#,
        )
        .bind(id)
        .fetch_one(&state.pool)
        .await?,
    ))
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
