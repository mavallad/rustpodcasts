use serde::Serialize;
use chrono::NaiveDate;
use actix_web::web;
use sqlx::PgPool;
use crate::repositories::channel_from_db;

pub async fn channel(path_channel_id: web::Path<i64>, pool: web::Data<PgPool>) -> web::Json<Option<ChannelWithEpisodes>> {
    let channel_id = *path_channel_id;
    log::info!("Asking for the channel {}", channel_id);
    web::Json(channel_from_db(channel_id, pool.get_ref()).await)
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

impl ChannelWithEpisodes {
    pub fn new(id: i64, name: String, description: String, host: Option<String>, url: String,
            lang: String, icon_path: String, active: bool, episodes: Vec<Episode>) -> ChannelWithEpisodes {
        ChannelWithEpisodes { id, name, description, host, url, lang, icon_path, active, episodes }
    }

    pub fn add_episode(&mut self, episode: Episode) {
        self.episodes.push(episode);
    }
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

impl Episode {
    pub fn new(id: i64, title: String, description: String, lang: String, url: String,
            date_published: NaiveDate, duration_seconds: u32, icon_path: Option<String>) -> Episode {
        Episode { id, title, description, lang, url, date_published, duration_seconds, icon_path }
    }
}
