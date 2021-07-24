#[macro_use]
extern crate lazy_static;

mod model;
mod util;

fn main() {
    dotenv::dotenv().ok();
    println!("Hello, world!");
}
