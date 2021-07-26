use std::{convert::Infallible, env};

use hyper::Body;
use routerify::Router;

use super::misc::serve_style_feed;
use super::vector_api::serve_vector;

pub fn get_router() -> Router<Body, Infallible> {
    Router::builder()
        .get(format!("/data/{0}/:zoom/:x/:y.pbf", env::var("TS_MAP_NAME").unwrap_or("default".to_string())), serve_vector)
        .get(format!("/styles/{0}/style.json", env::var("TS_MAP_NAME").unwrap_or("default".to_string())), serve_style_feed)
        .build().unwrap()
}