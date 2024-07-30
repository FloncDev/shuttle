pub mod routers;
pub mod session;

use dotenvy::var;
pub use session::Session;
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
