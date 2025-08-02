use alloy::json_abi::JsonAbi;
use serde_json::Value;
use std::fs;
use std::sync::Arc;
use std::{env, path::Path};
use watcher::{
    config::WatcherConfig, orderbook::provider::OrderbookProvider, server::Server,
    types::ChainType, watchers::factory::FactoryWatcher,
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
    start_factory_watchers(config.clone(), db.clone()).await?;

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
    Ok(db)
}

fn start_server(config: WatcherConfig) {
    tokio::spawn(async move {
        Server::new("0.0.0.0:6060".into(), Arc::new(config))
            .run()
            .await;
    });
}

async fn start_factory_watchers(
    config: WatcherConfig,
    db: Arc<OrderbookProvider>,
) -> anyhow::Result<()> {
    let mut watchers = Vec::new();

    for evm_config in config.chains.evm {
        if !evm_config.rpc_url.is_empty() && !evm_config.contract_address.is_empty() {
            tracing::info!("Initializing {} factory watcher", evm_config.name);
            let json_abi = load_abi(Path::new("src/abi/escrow_factory.json"))?;

            let watcher = FactoryWatcher::new(
                evm_config.rpc_url.clone(),
                evm_config.contract_address.clone(),
                ChainType::Ethereum(evm_config.name.clone()),
                db.clone(),
                evm_config.start_block,
                &json_abi,
            )
            .await?;

            watchers.push(watcher);
        }
    }

    for starknet_config in config.chains.starknet {
        if !starknet_config.rpc_url.is_empty() && !starknet_config.contract_address.is_empty() {
            tracing::info!("Initializing {} factory watcher", starknet_config.name);
            let json_abi = load_abi(Path::new("src/abi/escrow_factory.json"))?;

            let watcher = FactoryWatcher::new(
                starknet_config.rpc_url.clone(),
                starknet_config.contract_address.clone(),
                ChainType::Starknet(starknet_config.name.clone()),
                db.clone(),
                starknet_config.start_block,
                &json_abi,
            )
            .await?;

            watchers.push(watcher);
        }
    }

    for mut watcher in watchers {
        let chain_name = watcher.chain_name().to_string();
        tokio::spawn(async move {
            if let Err(e) = watcher.start().await {
                tracing::error!("Factory watcher error for {}: {:?}", chain_name, e);
            }
        });
    }

    tracing::info!("All factory watchers started successfully");
    Ok(())
}

pub fn load_abi(path: &Path) -> anyhow::Result<JsonAbi> {
    let abi_content = fs::read_to_string(path)?;
    let full_json: Value = serde_json::from_str(&abi_content)?;

    let abi_array = full_json
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Missing 'abi' field in contract artifact"))?;

    let json_abi: JsonAbi = serde_json::from_value(Value::Array(abi_array.clone()))?;

    Ok(json_abi)
}
