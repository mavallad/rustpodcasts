use actix_web::web;
use serde::Serialize;
use sqlx::PgPool;

pub async fn list_channels(pool: web::Data<PgPool>) -> web::Json<Vec<ChannelSummary>> {
    let records = sqlx::query_as!(
            ChannelSummary,
            "select channels.id, channels.name, channels.description, channels.url, \
            channels.lang, channels.icon_path, channels.active, count(*) as total_episodes \
            from channels inner join episodes on channels.id = episodes.channel_id \
            group by channels.id, channels.name, channels.description, channels.url, \
            channels.lang, channels.icon_path, channels.active")
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
    total_episodes: Option<i64>,
    active: bool
}






/*
Possible queries
=====================
select channels.id, channels.name, channels.description, channels.url,
channels.lang, channels.icon_path, channels.active, count(*) as total_episodes
from channels inner join episodes on channels.id = episodes.channel_id
group by channels.id, channels.name, channels.description, channels.url,
channels.lang, channels.icon_path, channels.active;


with count_episodes as (
    select channel_id, count(*) as total_episodes
    from episodes
    group by channel_id)
select channels.id, channels.name, channels.description, channels.url,
channels.lang, channels.icon_path, channels.active, count_episodes.total_episodes
from channels inner join count_episodes on channels.id = count_episodes.channel_id;
*/
