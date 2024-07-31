use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
    AppState: axum::extract::FromRef<S>,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let State(app_state): State<AppState> =
            State::from_request_parts(parts, state).await.unwrap();

        let cookies = CookieJar::from_request_parts(parts, state).await.unwrap();

        let token = match (cookies.get("token"), parts.headers.get("Authorization")) {
            (Some(token), _) => token.value_trimmed(),
            (_, Some(token)) => token.to_str().unwrap(),
            (None, None) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Missing Authorization header/token",
                ))
            }
        }
        .to_string();

        sqlx::query_as!(
            User,
            r#"
                SELECT users.*
                FROM tokens
                JOIN users
                ON tokens.user_id = users.id
                WHERE token = $1
            "#,
            token
        )
        .fetch_one(&app_state.pool)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid Authorization token"))
    }
}
