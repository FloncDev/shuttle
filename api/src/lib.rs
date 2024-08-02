pub mod models;
pub mod routers;

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
