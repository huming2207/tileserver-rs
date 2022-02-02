use std::env;
use actix_web::{App, HttpServer, web};
use log::info;
use actix_web_static_files;

use crate::api::{misc::{serve_data_feed, serve_day_style_feed, serve_night_style_feed}, vector_api::serve_vector};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub async fn get_router() -> std::io::Result<()> {
    let addr_str = env::var("TS_API_ADDR").unwrap_or("127.0.0.1:8888".to_string());
    info!("Loading server at {0}", addr_str);

    let day_map_name = env::var("TS_MAP_DAY_NAME").unwrap_or("default_day".to_string());
    let night_map_name = env::var("TS_MAP_NIGHT_NAME").unwrap_or("default_night".to_string());
    let map_serv_name = env::var("TS_NAP_SERV_NAME").unwrap_or("default".to_string());

    HttpServer::new(move || {
        let static_generated = generate();
        App::new().service(
            web::scope(format!("/data/{0}", map_serv_name).as_str()).service(serve_vector)
        ).route(
            format!("/styles/{0}/style.json", day_map_name).as_str(), web::get().to(serve_day_style_feed)
        ).route(
            format!("/styles/{0}/style.json", night_map_name).as_str(), web::get().to(serve_night_style_feed)
        ).route(
            format!("/data/{0}.json", map_serv_name).as_str(), web::get().to(serve_data_feed)
        ).service(actix_web_static_files::ResourceFiles::new(
            "/", static_generated,
        ))
    }).bind(addr_str)?.run().await
}