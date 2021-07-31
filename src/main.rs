#[macro_use]
extern crate lazy_static;

mod model;
mod util;
mod api;

use log::{info, warn};
use mimalloc::MiMalloc;

use crate::api::router::get_router;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    match dotenv::dotenv() {
        Ok(_) => info!("Dotenv config loaded"),
        Err(_) => warn!("Dotenv config was not loaded, using system environment instead"),
    }

    get_router().await
}
