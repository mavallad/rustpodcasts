use async_trait::async_trait;

use super::{PodcastsRepository, ResultQuery, QueryError, from_db::to_channel_with_last_episode};
use crate::model::{EpisodeLast, ChannelWithLastEpisode};
use super::from_db::to_episode_last;

const SQL_LAST_EPISODES: &str =
r#"
select e.id, e.channel_id, e.title, c.name as channel_name, e.guests, e.duration_seconds,
e.lang, e.url, e.date_published, e.icon_path, c.icon_path as channel_icon_path,
case
  when length(e.description) > 300 then left(e.description, 300) || '...'
  else e.description
end as description_summary
from episodes e
inner join channels c on e.channel_id = c.id
order by e.date_published desc
limit 5
"#;

const SQL_ACTIVE_CHANNELS: &str =
r#"
with
last_episodes as (select distinct on (channel_id) id as episode_id, channel_id, title, url, date_published
  from episodes order by channel_id, date_published desc),
count_episodes as (select channel_id, count(*) as total_episodes from episodes group by channel_id)
select channels.id as channel_id, channels.name, channels.description, channels.url, channels.lang,
  channels.icon_path, channels.hosts, channels.active, channels.rust_centered,
  last_episodes.episode_id as last_episode_id, last_episodes.title as last_episode_title,
  last_episodes.url as last_episode_url, last_episodes.date_published as last_episode_date_published,
  count_episodes.total_episodes
from last_episodes
inner join count_episodes on last_episodes.channel_id = count_episodes.channel_id
inner join channels on count_episodes.channel_id = channels.id
where channels.rust_centered = true
and channels.active = true
order by last_episodes.date_published desc
limit 3
"#;

const SQL_ALL_CHANNELS: &str =
r#"
with
last_episodes as (select distinct on (channel_id) id as episode_id, channel_id, title, url, date_published
  from episodes order by channel_id, date_published desc),
count_episodes as (select channel_id, count(*) as total_episodes from episodes group by channel_id)
select channels.id as channel_id, channels.name, channels.description, channels.url, channels.lang,
  channels.icon_path, channels.hosts, channels.active, channels.rust_centered,
  last_episodes.episode_id as last_episode_id, last_episodes.title as last_episode_title,
  last_episodes.url as last_episode_url, last_episodes.date_published as last_episode_date_published,
  count_episodes.total_episodes
from last_episodes
inner join count_episodes on last_episodes.channel_id = count_episodes.channel_id
inner join channels on count_episodes.channel_id = channels.id
where channels.id != 1
order by last_episodes.date_published desc
"#;

#[derive(Clone)]
pub struct PostgresPodcastsRepository {
    pool: sqlx::PgPool,
}

impl PostgresPodcastsRepository {
    pub fn new(pool: sqlx::postgres::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PodcastsRepository for PostgresPodcastsRepository {
    async fn get_last_episodes(&self) -> ResultQuery<Vec<EpisodeLast>> {
        let res_query = sqlx::query_as(SQL_LAST_EPISODES)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| QueryError::new(SQL_LAST_EPISODES.to_owned(), e.to_string())
        );
        match res_query {
            Ok(episodes_db) => {
                let episodes_last = episodes_db.into_iter().map(|e| to_episode_last(e)).collect();
                Ok(episodes_last)
            },
            Err(e) => Err(e)
        }
    }

    async fn get_rust_active_channels(&self) -> ResultQuery<Vec<ChannelWithLastEpisode>> {
        let res_query = sqlx::query_as(SQL_ACTIVE_CHANNELS)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| QueryError::new(SQL_ACTIVE_CHANNELS.to_owned(), e.to_string())
        );
        match res_query {
            Ok(channels_db) => {
                let channels = channels_db.into_iter().map(|c| to_channel_with_last_episode(c)).collect();
                Ok(channels)
            },
            Err(e) => Err(e)
        }
    }

    async fn get_all_channels(&self) -> ResultQuery<Vec<ChannelWithLastEpisode>> {
        let res_query = sqlx::query_as(SQL_ALL_CHANNELS)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| QueryError::new(SQL_ALL_CHANNELS.to_owned(), e.to_string())
        );
        match res_query {
            Ok(channels_db) => {
                let channels = channels_db.into_iter().map(|c| to_channel_with_last_episode(c)).collect();
                Ok(channels)
            },
            Err(e) => Err(e)
        }
    }
}
