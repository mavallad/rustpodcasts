use actix_web::web;
use chrono::NaiveDate;
use serde::Serialize;
use sqlx::PgPool;

pub async fn last_episodes(pool: web::Data<PgPool>) -> web::Json<Vec<LastEpisode>> {
    let records = sqlx::query_as!(
            LastEpisode,
            "select channels.id as channel_id, channels.name as channel_name, channels.icon_path as channel_icon_path, \
            episodes.id, episodes.title, episodes.description, episodes.lang, episodes.url, episodes.date_published, episodes.duration_seconds \
            from channels inner join episodes \
            on channels.id = episodes.channel_id \
            order by episodes.date_published desc \
            limit 5")
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
pub struct LastEpisode {
    id: i64,
    channel_id: i64,
    channel_name: String,
    title: String,
    description: Option<String>,
    lang: String,
    url: String,
    channel_icon_path: String,
    date_published: Option<NaiveDate>,
    duration_seconds: Option<i32>
}
