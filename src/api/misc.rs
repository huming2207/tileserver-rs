use std::{convert::Infallible, env, fs};
use lazy_static::lazy_static;

use hyper::{Body, Request, Response};

lazy_static! {
    static ref STYLE_JSON_FEED: String = fs::read_to_string(env::var("TS_MAP_STYLE").unwrap_or("style.json".to_string())).unwrap();
}

pub async fn serve_style_feed(_req: Request<Body>) -> Result<Response<Body>, Infallible> { 
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(STYLE_JSON_FEED.as_str())).unwrap())
}