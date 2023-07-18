use actix_web::{
    get,
    web::{self},
    HttpResponse,
    Responder
};
use shuttle_runtime::tracing::log;
use lib::model::{EpisodeLast, ChannelWithLastEpisode};
use crate::common::AppState;
use lib::repository::PodcastsRepository;
use tera::Context;

#[get("/index.html")]
pub async fn index(state: web::Data<AppState>) -> impl Responder {
    let repository: &dyn PodcastsRepository = &state.repository;
    let last_episodes_data = match repository.get_last_episodes().await {
        Ok(episodes) => episodes,
        Err(query_error) => { log::error!("{}", query_error); vec![] }
    };
    let active_channels_data = match repository.get_active_channels().await {
        Ok(channels) => channels,
        Err(query_error) => { log::error!("{}", query_error); vec![] }
    };
    let mut ctx = Context::new();
    ctx.insert("page_id", "index");
    ctx.insert("last_episodes", &last_episodes_data);
    ctx.insert("active_channels", &active_channels_data);
    let rendered = state.tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
