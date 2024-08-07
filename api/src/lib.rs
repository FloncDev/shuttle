pub mod models;
pub mod routers;

use axum::{http::StatusCode, response::IntoResponse};
use dotenvy::var;
pub use models::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

impl AppState {
    pub async fn new() -> Self {
        let pool = PgPoolOptions::new()
            .connect(
                var("DATABASE_URL")
                    .expect("DATABASE_URL is not set")
                    .as_str(),
            )
            .await
            .expect("Error connecting to database");

        AppState { pool }
    }
}

pub type Result<T> = core::result::Result<T, ApiError>;

pub enum ApiError {
    DatabaseError(sqlx::Error),
    GameNotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::DatabaseError(err) => {
                tracing::error!("An database error has occured. {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something internally went wrong.",
                )
            }
            ApiError::GameNotFound => (StatusCode::NOT_FOUND, "Specified game could not be found."),
        }
        .into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::DatabaseError(err)
    }
}
