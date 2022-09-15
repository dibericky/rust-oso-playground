#[macro_use]
extern crate rocket;

mod db;
mod models;
mod server;

#[tokio::main]
async fn main() {
    if let Err(err) = server::run().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
