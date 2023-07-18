use actix_web::{
    get,
    web::{self, Json},
    Result
};
use shuttle_runtime::tracing::log;
use lib::model::{EpisodeLast, ChannelWithLastEpisode};
use crate::common::AppState;
use lib::repository::PodcastsRepository;

#[get("/last_episodes")]
pub async fn last_episodes(state: web::Data<AppState>) -> Result<Json<Vec<EpisodeLast>>> {
    let repository: &dyn PodcastsRepository = &state.repository;
    match repository.get_last_episodes().await {
        Ok(episodes) => Ok(Json(episodes)),
        Err(query_error) => { log::error!("{}", query_error); Ok(Json(vec![])) }
    }
}

#[get("/active_channels")]
pub async fn active_channels(state: web::Data<AppState>) -> Result<Json<Vec<ChannelWithLastEpisode>>> {
    let repository: &dyn PodcastsRepository = &state.repository;
    match repository.get_active_channels().await {
        Ok(channels) => Ok(Json(channels)),
        Err(query_error) => { log::error!("{}", query_error); Ok(Json(vec![])) }
    }
}
