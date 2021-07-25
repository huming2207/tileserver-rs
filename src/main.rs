#[macro_use]
extern crate lazy_static;

mod model;
mod util;
mod api;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    println!("Hello, world!");
}
