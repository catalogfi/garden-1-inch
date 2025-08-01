use std::env;
use std::sync::Arc;
use watcher::{
    config::WatcherConfig, orderbook::provider::OrderbookProvider, server::Server,
    types::ChainType, watcher::Watcher,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = load_config();

    let db = init_db(&config.core.db_url)
        .await
        .expect("Failed to initialize database");
    let db = Arc::new(db);

    start_server(config.clone());
    start_watchers(config, db).await?;

    tokio::signal::ctrl_c().await?;
    Ok(())
}

fn load_config() -> WatcherConfig {
    let config_file = if env::args().any(|arg| arg == "--local") {
        "local_config.toml"
    } else {
        "config.toml"
    };
    WatcherConfig::from_toml(config_file)
}

async fn init_db(db_url: &str) -> anyhow::Result<OrderbookProvider> {
    let db = OrderbookProvider::from_db_url(db_url)
        .await
        .expect("Failed to connect to database");
    // No need to create tables here, as the database should already be set up
    // db.create_tables().await.expect("Failed to create tables");
    Ok(db)
}

fn start_server(config: WatcherConfig) {
    tokio::spawn(async move {
        Server::new("0.0.0.0:6060".into(), Arc::new(config))
            .run()
            .await;
    });
}

async fn start_watchers(config: WatcherConfig, db: Arc<OrderbookProvider>) -> anyhow::Result<()> {
    let mut watchers = Vec::new();

    if !config.rpc.ethereum_rpc.is_empty() && !config.contracts.ethereum_contract_address.is_empty()
    {
        watchers.push(
            Watcher::new(
                config.rpc.ethereum_rpc.clone(),
                config.contracts.ethereum_contract_address.clone(),
                ChainType::Ethereum,
                db.clone(),
                config.contracts.ethereum_start_block,
            )
            .await?,
        );
    }

    if !config.contracts.starknet_contract_address.is_empty() {
        watchers.push(
            Watcher::new(
                config.rpc.starknet_rpc.clone(),
                config.contracts.starknet_contract_address.clone(),
                ChainType::Starknet,
                db.clone(),
                config.contracts.starknet_start_block,
            )
            .await?,
        );
    }

    for mut watcher in watchers {
        tokio::spawn(async move {
            if let Err(e) = watcher.start().await {
                tracing::error!("Watcher error: {:?}", e);
            }
        });
    }

    Ok(())
}
