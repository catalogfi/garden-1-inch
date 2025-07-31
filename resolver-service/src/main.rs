use std::time::Duration;

use anyhow::Result;

use crate::oneinch::orders::OrdersClient;
use crate::order_mapper::OrderMapper;
use crate::resolver::create_resolver;
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

    let mut order_mapper_builder = OrderMapper::builder();

    order_mapper_builder = order_mapper_builder
        .with_order_client(order_client)
        .with_poll_interval(Duration::from_secs(settings.poll_interval))
        .with_action_ttl(Duration::from_secs(300)); // 5 minutes TTL for action reprocessing
    
    for (chain_name, chain_settings) in settings.chains {
        let chain_id = chain_settings.chain_id.clone();
        let assets = chain_settings.assets.clone();

        let resolver = create_resolver(&chain_settings).await;
        order_mapper_builder = order_mapper_builder.add_chain_resolver(chain_id.clone(), resolver);
        order_mapper_builder = order_mapper_builder.add_supported_assets(chain_id.clone(), assets.clone());

        tracing::info!(chain_name=?chain_name, chain_id = chain_id, assets = ?assets, "Added chain");
    }

    let mut order_mapper = order_mapper_builder.build()?;

    // Run order mapper (which now handles all resolver calls directly)
    order_mapper.run().await;

    // Keep the main thread alive
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutting down...");
    Ok(())
}
