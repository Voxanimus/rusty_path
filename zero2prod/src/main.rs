use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use zero2prod::{
    configurations::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configurations = get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect_lazy_with(configurations.database.with_db());

    let address = format!(
        "{}:{}",
        configurations.application.host, configurations.application.port
    );

    let listener = TcpListener::bind(address).await?;
    tracing::info!("Starting service, listening on {}", listener.local_addr()?);
    run(listener, connection_pool)?.await
}
