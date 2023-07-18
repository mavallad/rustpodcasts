pub mod postgres;

use std::fmt;
use crate::model::{EpisodeLast, ChannelWithLastEpisode};

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
    async fn  get_active_channels(&self) -> ResultQuery<Vec<ChannelWithLastEpisode>>;
    // async fn get_film(&self, id: &Uuid) -> FilmResult<Film>;
    // async fn create_film(&self, id: &CreateFilm) -> FilmResult<Film>;
    // async fn update_film(&self, id: &Film) -> FilmResult<Film>;
    // async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid>;
}
