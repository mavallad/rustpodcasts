mod common;

use sqlx::{PgConnection, Connection};
use chrono::NaiveDate;
use rustpodcasts::configuration::get_configuration;
use reqwest::header::HeaderValue;
use common::spawn_app;

#[tokio::test]
async fn list_channels_returns_200_with_list_of_channels() {
    let app = spawn_app().await;
    let configuration = get_configuration("tests/config/configuration.yaml").expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::query!(
            r#"
            INSERT INTO channels (name, host, description, url, lang, icon_path, active)
            VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING
            "#,
            "::name::".to_string(),
            "::host::".to_string(),
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
    let expected_body = r#"[{"id":1,"name":"::name::","description":"::description::","url":"::url::","lang":"en","icon_path":"::icon_path::","total_episodes":1,"active":true}]"#;
    assert_eq!(expected_body, response.text().await.expect("Failed to read text of response"));
}
