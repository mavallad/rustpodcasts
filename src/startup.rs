use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;
use std::net:: TcpListener;
use crate::routes::{last_episodes, health_check, channels_last_episode};

pub fn run(
        listener:TcpListener,
        db_pool: PgPool
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/channels_last_episode", web::get().to(channels_last_episode))
        .route("/last_episodes", web::get().to(last_episodes))
        .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
