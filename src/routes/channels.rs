use actix_web::web;
use chrono::NaiveDate;
use serde::Serialize;
use sqlx::{PgPool};

pub async fn list_channel_summaries(pool: web::Data<PgPool>) -> web::Json<Vec<ChannelSummary>> {
    let records = sqlx::query_as!(
            ChannelSummary,
            "select distinct on (e.channel_id) e.id, c.id, c.name, c.description, c.url, c.lang, c.icon_path, c.active, e.channel_id as last_episode_id, e.date_emitted as last_episode_date from episodes e order by e.date_emitted desc"
            "select  null as last_episode_title, null::DATE as last_episode_date, 0::INT as count_episodes from channels c")
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
    id: i64,
    name: String,
    description: String,
    url: String,
    lang: String,
    icon_path: String,
    active: bool,
    last_episode_title: Option<String>,
    last_episode_date: Option<NaiveDate>,
    count_episodes: Option<i32>
}
