use actix_web::{
    get,
    web::{self, Json},
    Result, HttpResponse, Responder
};
use chrono::NaiveDate;
use serde::Serialize;
use shuttle_runtime::tracing::log;
use crate::common::{AppState, QueryError};
use sqlx::{Executor, FromRow, PgPool};
use tera::Context;

type ResultQuery<T> = std::result::Result<T, QueryError>;

const SQL: &str = r#"
                    with
                    last_episodes as (select distinct on (channel_id) id as episode_id, channel_id, title, url, date_published from episodes order by channel_id, date_published),
                    count_episodes as (select channel_id, count(*) as total_episodes from episodes group by channel_id)
                    select channels.id as channel_id, channels.name, channels.lang, channels.icon_path,
                    last_episodes.episode_id as last_episode_id, last_episodes.title as last_episode_title,
                    last_episodes.date_published as last_episode_date_published, count_episodes.total_episodes
                    from last_episodes
                    inner join count_episodes on last_episodes.channel_id = count_episodes.channel_id
                    inner join channels on count_episodes.channel_id = channels.id
                    "#;

#[get("/api/channels_last_episode")]
pub async fn json(state: web::Data<AppState>) -> Result<Json<Vec<ChannelWithLastEpisode>>> {
    match data(&state).await {
        Ok(channels) => Ok(Json(channels)),
        Err(query_error) => { log::error!("{}", query_error); Ok(Json(vec![])) }
    }
}

#[get("/channels_last_episode")]
pub async fn html(state: web::Data<AppState>) -> impl Responder {
    let mut ctx = Context::new();
    let data = match data(&state).await {
        Ok(channels) => channels,
        Err(query_error) => { log::error!("{}", query_error); vec![] }
    };
    ctx.insert("name", "pepe");
    let rendered = state.tera.render("channels_last_episode.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn data(state: &web::Data<AppState>) -> ResultQuery<Vec<ChannelWithLastEpisode>> {
    sqlx::query_as(SQL)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| QueryError::new(SQL.to_owned(), e.to_string())
    )
}

#[derive(Serialize, FromRow)]
pub struct ChannelWithLastEpisode {
    channel_id: i64,
    name: String,
    lang: String,
    icon_path: String,
    last_episode_id: i64,
    last_episode_title: String,
    last_episode_date_published: NaiveDate,
    total_episodes: Option<i64>
}