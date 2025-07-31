use std::{env, sync::Arc};

use tracing::info;
use tracing_subscriber::fmt;
use watcher::{config::WatcherConfig, orderbook::orderbook::OrderbookProvider};

#[tokio::main]
async fn main() {
    fmt::init();
    let args: Vec<String> = env::args().collect();

    let local_config = if args.contains(&"--local".to_string()) {
        "local_config.toml"
    } else {
        "config.toml"
    };

    let config = WatcherConfig::from_toml(local_config);
    info!("Loaded configuration: {:#?}", config);

    let db = Arc::new(OrderbookProvider::from_db_url(&config.core.db_url).await.expect("Failed to connect to database"));
    db.create_tables()
        .await
        .expect("Failed to create database tables");

    info!("Database tables created successfully");
}
