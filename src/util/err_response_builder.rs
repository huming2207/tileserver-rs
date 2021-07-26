use hyper::{Body, Response};

pub fn build_not_found_response() -> Response<Body> {
    Response::builder().status(404).body(Body::from("Not Found")).unwrap()
}