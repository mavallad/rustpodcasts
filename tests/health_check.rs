use sqlx::{PgConnection, Connection};
use rustpodcasts::configuration::get_configuration;
use std::net:: TcpListener;
use reqwest::header::HeaderValue;
use rustpodcasts::startup::run;
use sqlx::PgPool;

#[tokio:: test]
async fn health_check_works() {
    let app = spawn_app().await;
    
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


#[tokio::test]
async fn list_channels_returns_200_with_list_of_channels() {
    let app = spawn_app().await;
    let configuration = get_configuration("tests/config/configuration.yaml").expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    println!("CONNECTION: {}", configuration.database.connection_string());
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::query!(
            r#"
            INSERT INTO channels (name, description, url, lang, icon_path, active)
            VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING
            "#,
            "::name::".to_string(),
            "::description::".to_string(),
            "::url::".to_string(),
            "::lang::".to_string(),
            "::icon_path::".to_string(),
            true
            )
            .execute(&mut connection)
            .await
            .expect("Failed to insert channel");
            
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("{}/list_channels", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    let response_headers = response.headers();

    assert!(response.status().is_success());
    assert_eq!(
        Some(&HeaderValue::from_static("application/json")),
        response_headers.get(reqwest::header::CONTENT_TYPE)
    );
    let expected_body = r#"[{"id":1,"name":"::name::","description":"::description::","url":"::url::","lang":"::lang::","icon_path":"::icon_path::","active":true,"last_episode_title":null,"last_episode_date":null,"count_episodes":0}]"#;
    assert_eq!(expected_body, response.text().await.expect("Failed to read text of response"));
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
    
async fn spawn_app() -> TestApp {
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
