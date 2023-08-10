pub mod postgres;
mod from_db;

use std::fmt;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use crate::model::{EpisodeLast, ChannelWithLastEpisode};

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
pub struct EpisodeLastDb {
    id: i64,
    channel_id: i64,
    channel_name: String,
    title: String,
    guests: Option<String>,
    description_summary: String,
    lang: String,
    url: String,
    date_published: NaiveDate,
    duration_seconds: i32,
    icon_path: Option<String>,
    channel_icon_path: Option<String>
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct ChannelWithLastEpisodeDb {
    channel_id: i64,
    name: String,
    description: String,
    url: String,
    lang: String,
    icon_path: Option<String>,
    hosts: Option<String>,
    last_episode_id: i64,
    last_episode_title: String,
    last_episode_url: String,
    last_episode_date_published: NaiveDate,
    total_episodes: Option<i64>
}

#[derive(Debug, Clone)]
pub struct QueryError {
    pub sql: String,
    pub error: String
}

impl QueryError {
    pub fn new(sql: String, error: String) -> QueryError {
        QueryError { sql, error }
    }
}
impl std::error::Error for QueryError {}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error executing query {}.\nError: {}", self.sql, self.error)
    }
}

pub type ResultQuery<T> = std::result::Result<T, QueryError>;

#[async_trait::async_trait]
pub trait PodcastsRepository: Send + Sync + 'static {
    async fn get_last_episodes(&self) -> ResultQuery<Vec<EpisodeLast>>;
    async fn  get_rust_active_channels(&self) -> ResultQuery<Vec<ChannelWithLastEpisode>>;
    // async fn get_film(&self, id: &Uuid) -> FilmResult<Film>;
    // async fn create_film(&self, id: &CreateFilm) -> FilmResult<Film>;
    // async fn update_film(&self, id: &Film) -> FilmResult<Film>;
    // async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid>;
}
