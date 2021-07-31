use std::env;
use actix_web::{App, HttpServer, web};
use log::info;

use crate::api::{misc::{serve_day_style_feed, serve_night_style_feed}, vector_api::serve_vector};

pub async fn get_router() -> std::io::Result<()> {
    let addr_str = env::var("TS_API_ADDR").unwrap_or("127.0.0.1:8888".to_string());
    info!("Loading server at {0}", addr_str);

    let day_map_name = env::var("TS_MAP_DAY_NAME").unwrap_or("default".to_string());
    let night_map_name = env::var("TS_MAP_NIGHT_NAME").unwrap_or("default".to_string());

    HttpServer::new(move || {
        App::new().service(
            web::scope(format!("/data/{0}", day_map_name).as_str()).service(serve_vector)
        ).service(
            web::scope(format!("/data/{0}", night_map_name).as_str()).service(serve_vector)
        ).route(
            format!("/styles/{0}/style.json", day_map_name).as_str(), web::get().to(serve_day_style_feed)
        ).route(
            format!("/styles/{0}/style.json", night_map_name).as_str(), web::get().to(serve_night_style_feed)
        )
    }).bind(addr_str)?.run().await
}