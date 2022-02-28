use actix_web::web;
use chrono::NaiveDate;
use serde::Serialize;

pub async fn list_channel_summaries() -> web::Json<Vec<ChannelSummary>> {
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
pub struct ChannelSummary {
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
