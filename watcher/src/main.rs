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

    for evm_config in config.chains.evm {
        if !evm_config.rpc_url.is_empty() && !evm_config.contract_address.is_empty() {
            tracing::info!("Initializing {} watcher", evm_config.name);

            let watcher = Watcher::new(
                evm_config.rpc_url.clone(),
                evm_config.contract_address.clone(),
                ChainType::Ethereum(evm_config.name.clone()),
                db.clone(),
                evm_config.start_block,
            )
            .await?;

            watchers.push(watcher);
        }
    }

    for starknet_config in config.chains.starknet {
        if !starknet_config.rpc_url.is_empty() && !starknet_config.contract_address.is_empty() {
            tracing::info!("Initializing {} watcher", starknet_config.name);

            let watcher = Watcher::new(
                starknet_config.rpc_url.clone(),
                starknet_config.contract_address.clone(),
                ChainType::Starknet(starknet_config.name.clone()),
                db.clone(),
                starknet_config.start_block,
            )
            .await?;

            watchers.push(watcher);
        }
    }

    for mut watcher in watchers {
        let chain_name = watcher.chain_name().to_string();
        tokio::spawn(async move {
            if let Err(e) = watcher.start().await {
                tracing::error!("Watcher error for {}: {:?}", chain_name, e);
            }
        });

        
    }

    tracing::info!("All watchers started successfully");

    // TODO: Add cron job for database polling
    // - Check for pending orders
    // - Handle orders that are source filled but not source settled or refunded

    Ok(())
}
