use axum::{
    extract::{Path, State},
    routing::{get, patch, post},
    Json, Router,
};

use crate::{AppState, SoloGame, User};

async fn get_games(State(state): State<AppState>) -> Result<Json<Vec<SoloGame>>, ()> {
    let games: Vec<SoloGame> = sqlx::query_as(
        r#"
            select g.id, g.created_at, g.first_to, g.sets,
                p1.id as p1_id,
                p1.name as p1_name,
                p1.created_at as p1_created_at,
                p2.id as p2_id,
                p2.name as p2_name,
                p2.created_at as p2_created_at
            from solo_games as g
            join users p1 on g.player_one = p1.id
            join users p2 on g.player_two = p2.id
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    Ok(Json(games))
}

async fn get_game(State(state): State<AppState>, Path(id): Path<i32>) -> Json<SoloGame> {
    let game: SoloGame = sqlx::query_as(
        r#"
            select g.id, g.created_at, g.first_to, g.sets,
                p1.id as p1_id,
                p1.name as p1_name,
                p1.created_at as p1_created_at,
                p2.id as p2_id,
                p2.name as p2_name,
                p2.created_at as p2_created_at
            from solo_games as g
            join users p1 on g.player_one = p1.id
            join users p2 on g.player_two = p2.id
            where g.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Json(game)
}

async fn post_game(State(state): State<AppState>, _: User, game: Json<SoloGame>) -> Json<SoloGame> {
    todo!();
}

async fn patch_game(State(state): State<AppState>, _: User) -> Json<SoloGame> {
    todo!();
}

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/solo/games", get(get_games))
        .route("/solo/game/:id", get(get_game))
        .route("/solo/game", post(post_game))
        .route("/solo/game", patch(patch_game))
}
