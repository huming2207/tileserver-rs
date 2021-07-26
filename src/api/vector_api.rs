use std::convert::Infallible;

use hyper::{Body, Request, Response};
use routerify::ext::RequestExt;

use crate::{model::tile::get_tile_data, util::err_response_builder::build_not_found_response};

pub async fn serve_vector(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let zoom = match req.param("zoom") {
        Some(str) => str,
        None => {
            return Ok(build_not_found_response());
        }
    };

    let x = match req.param("x") {
        Some(str) => str,
        None => {
            return Ok(build_not_found_response());
        }
    };

    let y = match req.param("y") {
        Some(str) => str,
        None => {
            return Ok(build_not_found_response());
        }
    };
    
    let tile = get_tile_data(zoom, x, y).unwrap();
    let resp = Response::builder()
        .header("Content-Type", "application/x-protobuf")
        .body(Body::from(tile)).unwrap();
    Ok(resp)
}