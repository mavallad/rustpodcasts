use sqlx::{PgConnection, Connection, PgPool};
use chrono::NaiveDate;
use rustpodcasts::configuration::get_configuration;
use std::net:: TcpListener;
use reqwest::header::HeaderValue;
use rustpodcasts::startup::run;

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
async fn channels_last_episode_returns_200_with_list_of_channels() {
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
            "en".to_string(),
            "::icon_path::".to_string(),
            true
            )
            .execute(&mut connection)
            .await
            .expect("Failed to insert channel");
            
    sqlx::query!(
        r#"
        INSERT INTO episodes (channel_id, title, guest, description, url, lang, date_published, duration_seconds, icon_path)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) ON CONFLICT DO NOTHING
        "#,
        1,
        "::title::".to_string(),
        Option::None as Option<String>,
        "::description::".to_string(),
        "::url::".to_string(),
        "en".to_string(),
        NaiveDate::from_ymd(2022, 10, 20),
        400,
        "::icon_path::".to_string()
        )
        .execute(&mut connection)
        .await
        .expect("Failed to insert episode");
    
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("{}/channels_last_episode", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    let response_headers = response.headers();

    assert!(response.status().is_success());
    assert_eq!(
        Some(&HeaderValue::from_static("application/json")),
        response_headers.get(reqwest::header::CONTENT_TYPE)
    );
    let expected_body = r#"[{"channel_id":1,"name":"::name::","lang":"en","icon_path":"::icon_path::","last_episode_id":1,"last_episode_title":"::title::","last_episode_date_published":"2022-10-20","total_episodes":1}]"#;
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
