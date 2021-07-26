#[macro_use]
extern crate lazy_static;

mod model;
mod util;
mod api;

use std::{env, net::SocketAddr};

use hyper::Server;
use log::{error, info, warn};
use mimalloc::MiMalloc;
use routerify::RouterService;

use crate::api::router::get_router;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    env_logger::init();

    match dotenv::dotenv() {
        Ok(_) => info!("Dotenv config loaded"),
        Err(_) => warn!("Dotenv config was not loaded, using system environment instead"),
    }

    let addr_str = env::var("TS_API_ADDR").unwrap_or("127.0.0.1:8888".to_string());
    info!("Loading server at {0}", addr_str);
    let router = get_router();
    let service = RouterService::new(router).unwrap();
    let addr: SocketAddr = addr_str.parse().expect(format!("Invalid server address for listening: {}", addr_str).as_str());
    let server = Server::bind(&addr).serve(service);
    info!("Server started!");
    if let Err(err) = server.await {
        error!("API server is down: {}", err);
    }
}
