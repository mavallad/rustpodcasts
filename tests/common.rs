use sqlx::{PgPool};
use rustpodcasts::configuration::get_configuration;
use std::net:: TcpListener;
use rustpodcasts::startup::run;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
    
pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let configuration = get_configuration("tests/config/configuration").expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio:: spawn(server);
    TestApp {
        address,
        db_pool: connection_pool
    }
}
