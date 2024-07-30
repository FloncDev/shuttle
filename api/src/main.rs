use api::{routers, AppState};
use axum::{extract::MatchedPath, http::Request, routing::get, Router};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().compact().init();

    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .merge(routers::get_router())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                tracing::info_span!("http_request", method = ?request.method(), matched_path)
            }),
        )
        .with_state(AppState::new().await);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    tracing::info!("Running server on http://{}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
