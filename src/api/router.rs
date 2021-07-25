use std::convert::Infallible;

use hyper::Body;
use routerify::Router;

pub fn get_router() -> Router<Body, Infallible> {
    Router::builder().build().unwrap()
}