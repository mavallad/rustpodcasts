use std::path::PathBuf;
use actix_web::{
    get,
    web::{self, Json, ServiceConfig},
    Result,
};
use actix_web::middleware::Logger;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{Executor, FromRow, PgPool};
use common::AppState;
use tera::Tera;

mod routes;
mod common;

#[get("/hello")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_runtime::main]
async fn actix_web(
            #[shuttle_shared_db::Postgres] pool: PgPool,
            #[shuttle_static_folder::StaticFolder(folder = "assets")] static_folder: PathBuf
        ) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!().run(&pool).await.expect("Migrations failed :(");

    let tera_templates_path = format!("{}/*.html", static_folder.to_string_lossy());
    let tera = Tera::new(&tera_templates_path).unwrap();

    let state = web::Data::new(AppState { pool, tera });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
            .wrap(Logger::default())
            .service(hello_world)
            .service(routes::channels_last_episode::json)
            .service(routes::channels_last_episode::html)
            .app_data(state)
        );
    };

    Ok(config.into())
}
