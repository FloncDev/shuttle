pub mod duo_game;
pub mod solo_game;
pub mod user;

use crate::AppState;
use axum::Router;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .merge(user::get_router())
        .merge(duo_game::get_router())
        .merge(solo_game::get_router())
}
