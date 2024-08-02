pub mod game;
pub mod user;

use crate::AppState;
use axum::Router;

pub fn get_router() -> Router<AppState> {
    Router::new().merge(user::get_router())
}
