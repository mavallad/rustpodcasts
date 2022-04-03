use actix_web::web;
use chrono::NaiveDate;
use serde::Serialize;
use sqlx::PgPool;

pub async fn channel(path_channel_id: web::Path<i64>, pool: web::Data<PgPool>) -> web::Json<Option<ChannelWithEpisodes>> {
    let channel_id = *path_channel_id;
    let records = sqlx::query_as!(
            ChannelEpisodeRow,
            "select channels.id, channels.name, channels.description, channels.host, \
            channels.url, channels.lang, channels.icon_path, channels.active, \
            episodes.id as episode_id, episodes.title as episode_title, \
            episodes.description as episode_description, episodes.lang as episode_lang,
            episodes.url as episode_url, episodes.date_published as episode_date_published, \
            episodes.duration_seconds as episode_duration, episodes.icon_path as episode_icon_path \
            from channels inner join episodes on channels.id = episodes.channel_id \
            where channels.id = $1", channel_id)
        .fetch_all(pool.get_ref())
        .await;

    web::Json(match records {
        Ok(rows) => transform_channel(rows),
        Err(error) => {
            eprintln!("ERROR IN QUERY: {}", error);
            Option::None
        }
    })
}

fn transform_channel(rows: Vec<ChannelEpisodeRow>) -> Option<ChannelWithEpisodes> {
    let mut channel_opt: Option<ChannelWithEpisodes> = Option::None;
    for row in rows {
        let episode_id = row.episode_id;
        let title = row.episode_title;
        let episode_description = row.episode_description;
        let episode_lang = row.episode_lang;
        let episode_url = row.episode_url;
        let episode_date_published = row.episode_date_published;
        let episode_duration = u32::try_from(row.episode_duration).unwrap();
        let episode_icon_path = row.episode_icon_path;
        let episode = Episode { id: episode_id, title: title, description: episode_description,
            lang: episode_lang, url: episode_url, date_published: episode_date_published,
            duration_seconds: episode_duration, icon_path: episode_icon_path };
        if let Some(the_channel) = &mut channel_opt {
            the_channel.episodes.push(episode);
        } else {
            let id = row.id;
            let name = row.name;
            let description = row.description;
            let host = row.host;
            let url = row.url;
            let lang = row.lang;
            let icon_path = row.icon_path;
            let active = row.active;
            let mut episodes = Vec::new();
            episodes.push(episode);
            channel_opt = Option::Some(ChannelWithEpisodes { id, name, description, host, url, lang, icon_path, active, episodes});
        }
    }
    channel_opt

}

struct ChannelEpisodeRow {
    id: i64,
    name: String,
    description: String,
    host: Option<String>,
    url: String,
    lang: String,
    icon_path: String,
    active: bool,
    episode_id: i64,
    episode_title: String,
    episode_description: String,
    episode_lang: String,
    episode_url: String,
    episode_date_published: NaiveDate,
    episode_duration: i32,
    episode_icon_path: Option<String>

}

#[derive(Serialize)]
pub struct ChannelWithEpisodes {
    id: i64,
    name: String,
    description: String,
    host: Option<String>,
    url: String,
    lang: String,
    icon_path: String,
    active: bool,
    episodes: Vec<Episode>
}

#[derive(Serialize)]
pub struct Episode {
    id: i64,
    title: String,
    description: String,
    lang: String,
    url: String,
    date_published: NaiveDate,
    duration_seconds: u32,
    icon_path: Option<String>
}
