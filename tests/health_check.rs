use std::net:: TcpListener;

use reqwest::header::HeaderValue;

#[tokio:: test]
async fn health_check_works() {
    let app_address = spawn_app();
    
    let client = reqwest::Client:: new();
    
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
    
    let client = reqwest::Client:: new();
    
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
    let expected_body = r#"[{"id":"34fasf-fadsf-dfas","name":"test channel","description":"a test channel","url":"https://test.channel.com/testing","lang":"en","icon_path":"images/icon/path/image.jpg","active":true,"last_episode_title":"my last episode","last_episode_date":"2020-10-20","number_episodes":10}]"#;
    assert_eq!(expected_body, response.text().await.expect("Failed to read text of response"));
}

fn spawn_app() -> String {
    let listener = TcpListener:: bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = rustpodcasts::run(listener).expect("Failed to bind address");
    let _ = tokio:: spawn(server);
    format! ("http://127.0.0.1:{}", port)
}
