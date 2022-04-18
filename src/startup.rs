use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use sqlx::PgPool;
use std::net:: TcpListener;
use crate::routes::{last_episodes, health_check, channels_last_episode, list_channels, channel};

pub fn run(
        listener:TcpListener,
        db_pool: PgPool
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .route("/health_check", web::get().to(health_check))
        .route("/channels_last_episode", web::get().to(channels_last_episode))
        .route("/last_episodes", web::get().to(last_episodes))
        .route("/list_channels", web::get().to(list_channels))
        .route("/channel/{channel_id}", web::get().to(channel))
        .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
