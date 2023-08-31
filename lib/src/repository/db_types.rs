use crate::model::{EpisodeLast, ChannelWithLastEpisode, ChannelWithEpisodes, Episode};
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

// Transformations from database types to model types
pub fn to_episode_last(eldb: EpisodeLastDb) -> EpisodeLast {    
    EpisodeLast {
        id: eldb.id,
        channel_id: eldb.channel_id,
        channel_name: eldb.channel_name,
        title: eldb.title,
        guests: eldb.guests,
        description_summary: eldb.description_summary,
        lang: eldb.lang,
        url: eldb.url,
        date_published: eldb.date_published,
        duration_seconds: eldb.duration_seconds,
        duration: format_hours_minutes_seconds(eldb.duration_seconds),
        icon_path: eldb.icon_path,
        channel_icon_path: eldb.channel_icon_path
    }
}

pub fn to_channel_with_last_episode(chdb: ChannelWithLastEpisodeDb) -> ChannelWithLastEpisode {
    ChannelWithLastEpisode {
        channel_id: chdb.channel_id,
        name: chdb.name,
        description: chdb.description,
        url: chdb.url,
        lang: chdb.lang,
        icon_path: chdb.icon_path,
        hosts: chdb.hosts,
        last_episode_id: chdb.last_episode_id,
        last_episode_title: chdb.last_episode_title,
        last_episode_url: chdb.last_episode_url,
        last_episode_date_published: chdb.last_episode_date_published,
        total_episodes: chdb.total_episodes.unwrap_or(0) as u32,
        active: chdb.active,
        rust_centered: chdb.rust_centered
    }
}

// pub channel_id: i64,
// pub title: String,
// pub guests: Option<String>,
// pub description: String,
// pub lang: String,
// pub url: String,
// pub date_published: NaiveDate,
// pub duration_seconds: i32,
// pub icon_path: Option<String>,

pub fn to_channel_with_episodes(vec_of_channel_and_episode: Vec<ChannelAndEpisodeDb>) -> Option<ChannelWithEpisodes> {
    if vec_of_channel_and_episode.is_empty() {
        None
    } else {
        let mut episodes: Vec<Episode> = Vec::with_capacity(vec_of_channel_and_episode.len());
        for channel_and_episode in &vec_of_channel_and_episode {
            if channel_and_episode.episode_id.is_some() {
                episodes.push(Episode {
                    id: channel_and_episode.episode_id.unwrap(),
                    channel_id: channel_and_episode.channel_id,
                    title: channel_and_episode.episode_title.clone().unwrap(),
                    guests: channel_and_episode.episode_guests.clone(),
                    description: channel_and_episode.episode_description.clone().unwrap(),
                    lang: channel_and_episode.episode_lang.clone().unwrap(),
                    url: channel_and_episode.episode_url.clone().unwrap(),
                    date_published: channel_and_episode.episode_date_published.clone().unwrap(),
                    duration_seconds: channel_and_episode.episode_duration_seconds.unwrap(),
                    duration: format_hours_minutes_seconds(channel_and_episode.episode_duration_seconds.unwrap()),
                    icon_path: channel_and_episode.episode_icon_path.clone()
                });
            }
        }
        let first_channel_and_episode = &vec_of_channel_and_episode[0];
        Some(ChannelWithEpisodes {
            channel_id: first_channel_and_episode.channel_id,
            name: first_channel_and_episode.channel_name.clone(),
            description: first_channel_and_episode.channel_description.clone(),
            url: first_channel_and_episode.channel_url.clone(),
            lang: first_channel_and_episode.channel_lang.clone(),
            icon_path: first_channel_and_episode.channel_icon_path.clone(),
            hosts: first_channel_and_episode.channel_hosts.clone(),
            active: first_channel_and_episode.channel_active,
            rust_centered: first_channel_and_episode.channel_rust_centered,
            total_episodes: episodes.len() as u32,
            episodes
        })
    }
}


fn format_hours_minutes_seconds(seconds: i32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    return if hours == 0 {
        format!("{:02}:{:02}", minutes, seconds)
    } else {
        format!("{:01}:{:02}:{:02}", hours, minutes, seconds)
    }
}

// Types to map database tables
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct EpisodeDb {
    id: i64,
    channel_id: i64,
    title: String,
    guests: Option<String>,
    description: String,
    lang: String,
    url: String,
    date_published: NaiveDate,
    duration_seconds: i32,
    icon_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct EpisodeLastDb {
    id: i64,
    channel_id: i64,
    channel_name: String,
    title: String,
    guests: Option<String>,
    description_summary: String,
    lang: String,
    url: String,
    date_published: NaiveDate,
    duration_seconds: i32,
    icon_path: Option<String>,
    channel_icon_path: Option<String>
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct ChannelWithLastEpisodeDb {
    channel_id: i64,
    name: String,
    description: String,
    url: String,
    lang: String,
    icon_path: Option<String>,
    hosts: Option<String>,
    last_episode_id: i64,
    last_episode_title: String,
    last_episode_url: String,
    last_episode_date_published: NaiveDate,
    total_episodes: Option<i64>,
    active: bool,
    rust_centered: bool
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct ChannelAndEpisodeDb {
    channel_id: i64,
    channel_name: String,
    channel_description: String,
    channel_url: String,
    channel_lang: String,
    channel_icon_path: Option<String>,
    channel_hosts: Option<String>,
    channel_active: bool,
    channel_rust_centered: bool,
    episode_id: Option<i64>,
    episode_title: Option<String>,
    episode_guests: Option<String>,
    episode_description: Option<String>,
    episode_lang: Option<String>,
    episode_url: Option<String>,
    episode_date_published: Option<NaiveDate>,
    episode_duration_seconds: Option<i32>,
    episode_icon_path: Option<String>
}
