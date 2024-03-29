use actix_web::{web::{self, ServiceConfig}, middleware::Logger};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::postgres::PgPool;
use lib::repository::postgres::PostgresPodcastsRepository;
use common::AppState;
use tera::Tera;

mod common;
mod routes;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!().run(&pool).await.expect("Migrations failed :(");

    let tera_templates_path_pattern = "statics/templates/*.html";
    let css_files_path = "statics/public/css/";
    let image_files_path = "statics/public/images/";
    let tera = Tera::new(&tera_templates_path_pattern).unwrap();

    let repository: PostgresPodcastsRepository = PostgresPodcastsRepository::new(pool);

    let state = web::Data::new(AppState { repository, tera });

    let config = move |cfg: &mut ServiceConfig| {
        cfg
        .service(web::scope("/api")
            .app_data(state.clone())
            .wrap(Logger::default())
            .service(routes::json::last_episodes)
            .service(routes::json::active_channels)
        )
        .service(actix_files::Files::new("/css", css_files_path).show_files_listing())
        .service(actix_files::Files::new("/images", image_files_path).show_files_listing())
        .service(web::scope("")
            .app_data(state.clone())
            .wrap(Logger::default())
            .service(routes::html::index)
            .service(routes::html::channels)
            .service(routes::html::channel)
            .service(routes::html::about)
        );
    };

    Ok(config.into())
}
