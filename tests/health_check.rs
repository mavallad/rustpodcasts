use sqlx::{PgConnection, Connection};
use rustpodcasts::configuration::get_configuration;
use std::net:: TcpListener;
use reqwest::header::HeaderValue;
use rustpodcasts::startup::run;

#[tokio:: test]
async fn health_check_works() {
    let app_address = spawn_app();
    
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


#[tokio::test]
async fn list_channels_returns_200_with_list_of_channels() {
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    println!("CONNECTION: {}", configuration.database.connection_string());
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::query!(
            r#"
            INSERT INTO channels (name, description, url, lang, icon_path, active)
            VALUES ($1, $2, $3, $4, $5, $6)
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
        .get(&format!("{}/list_channels", &app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    let response_headers = response.headers();

    assert!(response.status().is_success());
    assert_eq!(
        Some(&HeaderValue::from_static("application/json")),
        response_headers.get(reqwest::header::CONTENT_TYPE)
    );
    let expected_body = r#"[{"id":1,"name":"::name::","description":"::description::","url":"::url::","lang":"::lang::","icon_path":"::icon_path::","active":true,"last_episode_title":"::title::","last_episode_date":"2020-10-20","number_episodes":1}]"#;
    assert_eq!(expected_body, response.text().await.expect("Failed to read text of response"));
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to bind address");
    let _ = tokio:: spawn(server);
    format! ("http://127.0.0.1:{}", port)
}
