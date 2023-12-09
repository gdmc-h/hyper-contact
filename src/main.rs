#[macro_use]
extern crate serde_derive;

mod app;
mod models;
mod db;
mod schema;


#[tokio::main]
async fn main() {
    app::start().await;
}

