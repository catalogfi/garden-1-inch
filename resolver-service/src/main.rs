use std::time::Duration;

use anyhow::Result;

use crate::oneinch::orders::OrdersClient;
use crate::order_mapper::{create_chain_channel, OrderMapper};
use crate::resolver::Resolver;
use crate::settings::Settings;

mod oneinch;
mod resolver;
mod settings;
mod order_mapper;

const SETTINGS_PATH: &str = "Settings.toml";

#[tokio::main]
async fn main() -> Result<()> {

    tracing_subscriber::fmt::init();

    let settings = Settings::from_toml(SETTINGS_PATH)?;

    let oneinch_api_key = std::env::var("ONEINCH_API_KEY").expect("ONEINCH_API_KEY must be set");
    let url = settings.orders_url.clone();
    
    let order_client = OrdersClient::new(url.clone(), oneinch_api_key.clone());

    let mut  order_mapper_builder = OrderMapper::builder();

    order_mapper_builder = order_mapper_builder.with_order_client(order_client);
    order_mapper_builder = order_mapper_builder.with_poll_interval(Duration::from_secs(settings.poll_interval));

    let mut resolvers = Vec::new();
    
    for (chain_name, chain_settings) in settings.chains {
        let chain_id = chain_settings.chain_id;
        let assets = chain_settings.assets;

        let (sender, receiver) = create_chain_channel();

        let resolver = Resolver::new(receiver, chain_id.clone(), chain_settings.resolver_contract_address.clone(), chain_settings.provider.clone());
        order_mapper_builder = order_mapper_builder.add_chain_resolver(chain_id.clone(), sender);
        order_mapper_builder = order_mapper_builder.add_supported_assets(chain_id.clone(), assets.clone());

        resolvers.push(resolver);

        tracing::info!(chain_name=?chain_name, chain_id = chain_id, assets = ?assets, "Added chain");
    };


    let mut order_mapper = order_mapper_builder.build()?;


    // Start resolvers in background tasks
    let resolver_handles = run_resolvers(resolvers);

    // Run order mapper in the main task
    order_mapper.run().await;

    // Wait for resolvers to complete
    wait_for_resolvers(resolver_handles).await;

    // Keep the main thread alive
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutting down...");
    Ok(())
}


/// Runs all executors concurrently
fn run_resolvers(resolvers: Vec<Resolver>) -> Vec<tokio::task::JoinHandle<()>> {
    resolvers
        .into_iter()
        .map(|mut resolver| {
            tokio::spawn(async move {
                resolver.run().await;
            })
        })
        .collect()
}

async fn wait_for_resolvers(handles: Vec<tokio::task::JoinHandle<()>>) {
    for handle in handles {
        if let Err(e) = handle.await {
            tracing::error!("Resolver task failed: {}", e);
        }
    }
}
