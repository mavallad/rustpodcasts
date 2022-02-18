use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use chrono::NaiveDate;
use std::net:: TcpListener;
use serde::Serialize;

pub fn run(listener:TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/list_channels", web::get().to(list_channels))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn list_channels() -> web::Json<Vec<ChannelSummary>> {
    let channel = ChannelSummary {
        id: "34fasf-fadsf-dfas".to_string(),
        name: "test channel".to_string(),
        description: "a test channel".to_string(),
        url: "https://test.channel.com/testing".to_string(),
        lang: "en".to_string(),
        icon_path: "images/icon/path/image.jpg".to_string(),
        active: true,
        last_episode_title: "my last episode".to_string(),
        last_episode_date: NaiveDate::from_ymd(2020, 10, 20),
        number_episodes: 10        
    };
    let channels = vec![channel];
    web::Json(channels)
}

#[derive(Serialize)]
struct ChannelSummary {
    id: String,
    name: String,
    description: String,
    url: String,
    lang: String,
    icon_path: String,
    active: bool,
    last_episode_title: String,
    last_episode_date: NaiveDate,
    number_episodes: u8
}