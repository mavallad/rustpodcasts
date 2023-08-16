use crate::repository::{EpisodeLastDb, ChannelWithLastEpisodeDb};
use crate::model::{EpisodeLast, ChannelWithLastEpisode};

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
        total_episodes: chdb.total_episodes,
        active: chdb.active,
        rust_centered: chdb.rust_centered
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