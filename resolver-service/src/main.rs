use anyhow::Result;
use std::time::Duration;

use crate::oneinch::orders::OrdersClient;
use crate::order_mapper::{OrderMapperBuilder, create_chain_channel};
use crate::resolver::Resolver;

mod oneinch;
mod resolver;
mod order_mapper;

#[tokio::main]
async fn main() -> Result<()> {

    tracing_subscriber::fmt::init();

    let oneinch_api_key = std::env::var("ONEINCH_API_KEY").expect("ONEINCH_API_KEY must be set");
    let url = "https://api.1inch.dev/fusion-plus".to_string();
    let order_client = OrdersClient::new(url.clone(), oneinch_api_key.clone());

    // Create channels for different chains
    let (eth_sender, eth_receiver) = create_chain_channel();
    let (polygon_sender, polygon_receiver) = create_chain_channel();

    // Create resolvers for different chains
    let eth_resolver = Resolver::new(eth_receiver);
    let polygon_resolver = Resolver::new(polygon_receiver);

    // Start resolvers in separate tasks
    tokio::spawn(async move {
        eth_resolver.run();
    });

    tokio::spawn(async move {
        polygon_resolver.run();
    });

    // Build and configure the OrderMapper
    let mut order_mapper = OrderMapperBuilder::new()
        .with_order_client(order_client)
        .add_chain_resolver("1".to_string(), eth_sender) // Ethereum
        .add_chain_resolver("137".to_string(), polygon_sender) // Polygon
        .add_supported_assets("1".to_string(), vec![
            "0xA0b86a33E6441b8c4C8C1e9911b8c3e3f7b6b3b".to_string(), // Example ETH asset
            "0xB0b86a33E6441b8c4C8C1e9911b8c3e3f7b6b3c".to_string(), // Example ETH asset
        ])
        .add_supported_assets("137".to_string(), vec![
            "0x2791bca1f2de4661ed88a30c99a7a9449aa84174".to_string(), // USDC on Polygon
            "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".to_string(), // USDC.e on Polygon
        ])
        .with_poll_interval(Duration::from_secs(10))
        .build().map_err(|e| anyhow::anyhow!(e))?;

    order_mapper.run().await;

    tracing::info!("OrderMapper started successfully!");

    // Keep the main thread alive
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutting down...");

    Ok(())
}
