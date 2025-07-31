use crate::oneinch::{orders::{ActiveOrdersParams, OrdersClient}, quoter::{QuoteParams, QuoterClient}, relayer::{OrderInput, RelayerClient, SignedOrderInput}};

mod oneinch;


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let oneinch_api_key = std::env::var("ONEINCH_API_KEY").expect("ONEINCH_API_KEY must be set");
    let url = "https://api.1inch.dev/fusion-plus".to_string();
    let order_client = OrdersClient::new(url.clone(), oneinch_api_key.clone());

    let params = ActiveOrdersParams::new();
    let orders = order_client.get_active_orders(params).await;
    
    let quote_client = QuoterClient::new(url.clone(), oneinch_api_key.clone());

    let quote_params = QuoteParams::new(
        1,
        137,
        "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string(),
        "0x2791bca1f2de4661ed88a30c99a7a9449aa84174".to_string(),
        "1000000000000000000".to_string(),
        "0x0000000000000000000000000000000000000000".to_string(),
        false,
    );

    let quote = quote_client.get_quote(quote_params).await;
    println!("{:?}", quote);

    let relayer_client = RelayerClient::new(url.clone(), oneinch_api_key.clone());

    let order_input = OrderInput::new(
        "42".to_string(),
        "0x0000000000000000000000000000000000000001".to_string(),
        "0x0000000000000000000000000000000000000002".to_string(),
        "0x0000000000000000000000000000000000000003".to_string(),
        "100000000000000000000".to_string(),
        "100000000000000000000".to_string(),
    );
    let signed_order_input = SignedOrderInput::new(
        order_input,
        1,
        "0x0000000000000000000000000000000000000000".to_string(),
        "0x0000000000000000000000000000000000000000".to_string(),
        "0x0000000000000000000000000000000000000000".to_string(),
    );

    let result = relayer_client.submit_order(signed_order_input).await;
    println!("{:?}", result);
}
