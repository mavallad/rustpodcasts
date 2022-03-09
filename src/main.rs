use std::net:: TcpListener;
use rustpodcasts::startup::run;
use rustpodcasts::configuration::get_configuration;
use sqlx::PgPool;

#[tokio:: main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    // Renamed!
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
    .await
    .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener:: bind(address)?;

    run(listener, connection_pool)?.await
}
