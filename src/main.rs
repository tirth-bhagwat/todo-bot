use std::{env, sync::Arc};

mod bot;
mod models;
mod schema;

#[tokio::main]
async fn main() {
    bot::start().await;
}
