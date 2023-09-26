use actix_web::{
    get,
    web::{self},
    HttpResponse,
    Responder
};
use tracing::log;
use crate::common::AppState;
use lib::repository::PodcastsRepository;
use tera::Context;

#[get("/")]
pub async fn index(state: web::Data<AppState>) -> impl Responder {
    let repository = &state.repository;
    let lastest_episodes_data = match repository.get_last_episodes().await {
        Ok(episodes) => episodes,
        Err(query_error) => { log::error!("{}", query_error); vec![] }
    };
    let active_channels_data = match repository.get_rust_active_channels().await {
        Ok(active_channels) => active_channels,
        Err(query_error) => { log::error!("{}", query_error); vec![] }
    };
    let mut ctx = Context::new();
    ctx.insert("page_id", "index");
    let template_html;
    if !lastest_episodes_data.is_empty() {
        let last_episode = &lastest_episodes_data[0];
        let recent_episodes = &lastest_episodes_data[1..];
        ctx.insert("last_episode", last_episode);
        ctx.insert("recent_episodes", recent_episodes);
        ctx.insert("active_channels", &active_channels_data);
    
        template_html = "index.html";
    } else {
        template_html = "error.html";
    }
    let rendered = state.tera.render(template_html, &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/channels.html")]
pub async fn channels(state: web::Data<AppState>) -> impl Responder {
    let repository = &state.repository;
    let mut ctx = Context::new();
    ctx.insert("page_id", "channels");
    let template_html = match repository.get_all_channels().await {
        Ok(all_channels) => {
            let rust_channels = all_channels.iter()
                .filter(|c| c.rust_centered).collect::<Vec<_>>();
    
            let non_rust_channels = all_channels.iter()
                .filter(|c| !c.rust_centered).collect::<Vec<_>>();

            ctx.insert("rust_channels", &rust_channels);
            ctx.insert("non_rust_channels", &non_rust_channels);

            "channels.html"
        },
        Err(query_error) => {
            log::error!("{}", query_error);
            "error.html"
        }
    };
    let rendered = state.tera.render(template_html, &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/channel/{number}.html")]
pub async fn channel(path_channel_id: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let repository = &state.repository;
    let mut ctx = Context::new();
    ctx.insert("page_id", "channel");
    let channel_id = path_channel_id.into_inner();
    let template_html = match repository.get_channel(channel_id).await {
        Ok(opt_channel_with_episodes) => {
            match opt_channel_with_episodes {
                Some(channel_with_episodes) => {
                    ctx.insert("channel_with_episodes", &channel_with_episodes);
                    "channel.html"
                },
                None => {
                    log::warn!("No channel found with id {}", channel_id);
                    "error.html"
                }
            }
        },
        Err(query_error) => {
            log::error!("{}", query_error);
            "error.html"
        }
    };
    let rendered = state.tera.render(template_html, &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}


#[get("/about.html")]
pub async fn about(state: web::Data<AppState>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("page_id", "about");
    let rendered = state.tera.render("about.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
