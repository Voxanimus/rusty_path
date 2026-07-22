use axum::{
    Router,
    extract::Request,
    routing::{get, post},
    serve::Serve,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::Level;
use uuid::Uuid;

use crate::routes::{health_check, subscribe};

pub fn run(
    listener: TcpListener,
    connection: PgPool,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(connection)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request| {
                    let request_id = Uuid::new_v4();
                    tracing::info_span!(
                        "request",
                        %request_id,
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                })
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    let server = axum::serve(listener, app);

    Ok(server)
}
