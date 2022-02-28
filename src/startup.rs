use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net:: TcpListener;
use crate::routes::{list_channel_summaries, health_check};

pub fn run(listener:TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/list_channels", web::get().to(list_channel_summaries))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
