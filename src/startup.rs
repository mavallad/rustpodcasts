use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;
use std::net:: TcpListener;
use crate::routes::{list_channel_summaries, health_check};

pub fn run(
        listener:TcpListener,
        db_pool: PgPool
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/list_channels", web::get().to(list_channel_summaries))
        .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
