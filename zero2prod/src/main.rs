use sqlx::PgPool;
use tokio::net::TcpListener;
use zero2prod::{configurations::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configurations = get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPool::connect(&configurations.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configurations.app_port);

    let listener = TcpListener::bind(address).await?;
    run(listener, connection_pool)?.await
}
