use actix_web::web;
use chrono::NaiveDate;
use serde::Serialize;
use sqlx::PgPool;

pub async fn list_channel_summaries(pool: web::Data<PgPool>) -> web::Json<Vec<ChannelSummary>> {
    let records = sqlx::query_as!(
            ChannelSummary,
            "with \
            last_episodes as (select distinct on (channel_id) id as episode_id, channel_id, title, url, date_published from episodes order by channel_id, date_published), \
            count_episodes as (select channel_id, count(*) as total_episodes from episodes group by channel_id) \
            select channels.id as channel_id, channels.name, channels.lang, channels.icon_path, \
            last_episodes.episode_id as last_episode_id, last_episodes.title as last_episode_title, \
            last_episodes.date_published as last_episode_date_published, count_episodes.total_episodes \
            from last_episodes \
            inner join count_episodes on last_episodes.channel_id = count_episodes.channel_id \
            inner join channels on count_episodes.channel_id = channels.id")
        .fetch_all(pool.get_ref())
        .await;
    web::Json(match records {
        Ok(channels) => channels,
        Err(error) => {
            eprintln!("ERROR IN QUERY: {}", error);
            vec![]
        }
    })
}

#[derive(Serialize)]
pub struct ChannelSummary {
    channel_id: Option<i64>,
    name: Option<String>,
    lang: Option<String>,
    icon_path: Option<String>,
    last_episode_id: Option<i64>,
    last_episode_title: Option<String>,
    last_episode_date_published: Option<NaiveDate>,
    total_episodes: Option<i64>
}
