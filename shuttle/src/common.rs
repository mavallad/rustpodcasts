use lib::repository::postgres::PostgresPodcastsRepository;
use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    pub repository: PostgresPodcastsRepository,
    pub tera: Tera
}