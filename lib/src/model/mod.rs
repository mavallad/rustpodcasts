use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Episode {
    pub id: i64,
    pub channel_id: i64,
    pub title: String,
    pub guests: Option<String>,
    pub description: String,
    pub lang: String,
    pub url: String,
    pub date_published: NaiveDate,
    pub duration_seconds: i32,
    pub duration: String,
    pub icon_path: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct EpisodeLast {
    pub id: i64,
    pub channel_id: i64,
    pub channel_name: String,
    pub title: String,
    pub guests: Option<String>,
    pub description_summary: String,
    pub lang: String,
    pub url: String,
    pub date_published: NaiveDate,
    pub duration_seconds: i32,
    pub duration: String,
    pub icon_path: Option<String>,
    pub channel_icon_path: Option<String>
}

#[derive(Serialize, Debug)]
pub struct ChannelWithLastEpisode {
    pub channel_id: i64,
    pub name: String,
    pub description: String,
    pub url: String,
    pub lang: String,
    pub icon_path: Option<String>,
    pub hosts: Option<String>,
    pub last_episode_id: i64,
    pub last_episode_title: String,
    pub last_episode_url: String,
    pub last_episode_date_published: NaiveDate,
    pub total_episodes: u32,
    pub active: bool,
    pub rust_centered: bool
}

#[derive(Serialize, Debug)]
pub struct ChannelWithEpisodes {
    pub channel_id: i64,
    pub name: String,
    pub description: String,
    pub url: String,
    pub lang: String,
    pub icon_path: Option<String>,
    pub hosts: Option<String>,
    pub active: bool,
    pub rust_centered: bool,
    pub total_episodes: u32,
    pub episodes: Vec<Episode>
}
