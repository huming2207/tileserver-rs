use std::{env, fs};
use actix_web::HttpResponse;
use lazy_static::lazy_static;

lazy_static! {
    static ref DAY_STYLE_JSON_FEED: String = fs::read_to_string(env::var("TS_MAP_DAY_STYLE").unwrap_or("style_day.json".to_string())).unwrap();
    static ref NIGHT_STYLE_JSON_FEED: String = fs::read_to_string(env::var("TS_MAP_NIGHT_STYLE").unwrap_or("style_day.json".to_string())).unwrap();
}

pub(crate) async fn serve_day_style_feed() -> HttpResponse { 
    HttpResponse::Ok()
        .content_type("application/json")
        .body(NIGHT_STYLE_JSON_FEED.as_str())
}

pub(crate) async fn serve_night_style_feed() -> HttpResponse { 
    HttpResponse::Ok()
        .content_type("application/json")
        .body(NIGHT_STYLE_JSON_FEED.as_str())
}