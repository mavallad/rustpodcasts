use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Episode {
    id: i64,
    channel_id: i64,
    title: String,
    guests: Option<String>,
    description: String,
    lang: String,
    url: String,
    date_published: NaiveDate,
    duration_seconds: i32,
    icon_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct EpisodeLast {
    id: i64,
    channel_id: i64,
    channel_name: String,
    title: String,
    guests: Option<String>,
    description_summary: String,
    lang: String,
    url: String,
    date_published: NaiveDate,
    duration: String,
    icon_path: Option<String>,
    channel_icon_path: Option<String>
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ChannelWithLastEpisode {
    channel_id: i64,
    name: String,
    lang: String,
    icon_path: Option<String>,
    last_episode_id: i64,
    last_episode_title: String,
    last_episode_date_published: NaiveDate,
    total_episodes: Option<i64>
}
