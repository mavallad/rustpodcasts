use chrono::NaiveDate;
use sqlx::PgPool;

use crate::routes::{Episode, ChannelWithEpisodes};

pub async fn channel_from_db(channel_id: i64, pool: &PgPool) -> Option<ChannelWithEpisodes> {
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
        .fetch_all(pool)
        .await;

    match records {
        Ok(rows) => transform_channel(rows),
        Err(error) => {
            log::error!("Error in the query for channel {}: {}", channel_id, error);
            Option::None
        }
    }
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
        let episode = Episode::new(episode_id, title, episode_description,
            episode_lang, episode_url, episode_date_published,
            episode_duration, episode_icon_path);
        if let Some(the_channel) = &mut channel_opt {
            the_channel.add_episode(episode);
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
            channel_opt = Option::Some(ChannelWithEpisodes::new(id, name, description, host, url, lang, icon_path, active, episodes));
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
