use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::CookieJar;

use crate::AppState;

pub struct Session;

#[async_trait]
impl<S> FromRequestParts<S> for Session
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

        Ok(Session {})
    }
}
