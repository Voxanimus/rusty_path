use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
    serve::Serve,
};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub struct AppState {
    pub connection: PgPool,
}

pub fn run(
    listener: TcpListener,
    connection: PgPool,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let shared_state = Arc::new(AppState { connection });

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(shared_state);

    let server = axum::serve(listener, app);

    Ok(server)
}
