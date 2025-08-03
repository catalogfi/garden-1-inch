use crate::{
    orderbook::{OrderbookProvider, provider::OrderbookError},
    primitives::{
        ActiveOrderOutput, CrossChainOrder, GetActiveOrdersOutput, Meta, OrderInput, SecretInput,
        SecretResponse, SignedOrderInput,
    },
    server::Response,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use bigdecimal::BigDecimal;
use serde::Deserialize;

#[derive(Clone)]
pub struct HandlerState {
    pub orderbook: OrderbookProvider,
}

#[derive(Debug, Deserialize)]
pub struct ActiveOrdersQuery {
    page: Option<u64>,
    limit: Option<u64>,
}

pub async fn get_health() -> &'static str {
    "Online"
}

/// Get active (unmatched) orders
///
/// This endpoint retrieves all unmatched orders with pagination and filtering.
///
/// # Arguments
/// * `State(state)` - The handler state
/// * `Query(query)` - Query parameters for pagination and filtering
///
/// # Returns
/// * `Json<GetActiveOrdersOutput>` with paginated active orders
/// * `StatusCode::INTERNAL_SERVER_ERROR` if database error occurs
pub async fn get_active_orders(
    State(state): State<HandlerState>,
    Query(query): Query<ActiveOrdersQuery>,
) -> Result<Response<GetActiveOrdersOutput>, Response<()>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(100).min(500); // Max 500 items per page
    let offset = (page - 1) * limit;

    match state.orderbook.get_active_orders(limit, offset).await {
        Ok((orders, total_count)) => {
            let total_pages = (total_count + limit - 1) / limit;

            let meta = Meta {
                total_items: total_count,
                items_per_page: limit,
                total_pages,
                current_page: page,
            };

            let active_orders: Vec<ActiveOrderOutput> = orders
                .into_iter()
                .map(|order| {
                    // Convert secrets from JSON to Vec<SecretEntry>
                    let secrets = if order.secrets.is_null() {
                        vec![]
                    } else {
                        serde_json::from_value(order.secrets.clone()).unwrap_or_else(|_| vec![])
                    };

                    // Convert database order to OrderInput for API response
                    let order_input = OrderInput {
                        salt: order.salt.clone(),
                        maker_asset: order.maker_asset.clone(),
                        taker_asset: order.taker_asset.clone(),
                        maker: order.maker.clone(),
                        receiver: order.receiver.clone(),
                        making_amount: order.making_amount.clone(),
                        taking_amount: order.taking_amount.clone(),
                        maker_traits: order.maker_traits.clone(),
                    };

                    ActiveOrderOutput {
                        order_hash: order.order_hash.clone(),
                        signature: order.signature.clone(),
                        deadline: order.deadline as u64,
                        auction_start_date: order.auction_start_date.clone(),
                        auction_end_date: order.auction_end_date.clone(),
                        remaining_maker_amount: order.making_amount.to_string(),
                        extension: order.extension,
                        src_chain_id: order.src_chain_id as u64,
                        dst_chain_id: order.dst_chain_id as u64,
                        order: order_input,
                        taker: order.taker.clone(),
                        timelock: order.timelock.clone(),
                        taker_traits: order.taker_traits.clone(),
                        args: order.args.clone(),
                        order_type: order.order_type.clone(),
                        secrets,
                    }
                })
                .collect();

            let response = GetActiveOrdersOutput {
                meta,
                items: active_orders,
            };

            Ok(Response::ok(response))
        }
        Err(e) => {
            tracing::error!("Failed to retrieve active orders: {}", e);
            Err(Response::error(
                "Internal error",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

/// Submit a cross chain order that resolvers will be able to fill
///
/// This endpoint accepts a signed cross chain order and stores it for
/// distribution to resolvers who can fill the order.
///
/// # Arguments
/// * `State(state)` - The handler state
/// * `Json(signed_order)` - The signed order input containing order data and signature
///
/// # Returns
/// * `StatusCode::ACCEPTED` if the order is accepted for processing
/// * `StatusCode::BAD_REQUEST` if validation fails
pub async fn submit_order(
    State(state): State<HandlerState>,
    Json(signed_order): Json<SignedOrderInput>,
) -> Result<StatusCode, Response<()>> {
    // Validate the signed order
    if let Err(validation_error) = validate_signed_order(&signed_order) {
        return Err(Response::error(validation_error, StatusCode::BAD_REQUEST));
    }

    // Store the order in the database
    match state.orderbook.create_order(&signed_order).await {
        Ok(order_id) => {
            tracing::info!(
                order_id = order_id,
                order_hash = signed_order.order_hash,
                src_chain_id = signed_order.src_chain_id,
                maker = signed_order.order.maker,
                "Order Created"
            );
            Ok(StatusCode::ACCEPTED)
        }
        Err(e) => {
            tracing::error!("Failed to create order: {}", e);
            Err(Response::error(
                "Failed to create order",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

/// Get an order by its ID or hash
///
/// This endpoint retrieves a specific order from the database.
///
/// # Arguments
/// * `State(state)` - The handler state
/// * `Path(order_id)` - The order ID or hash to retrieve
///
/// # Returns
/// * `Json<CrossChainOrder>` if the order is found
/// * `StatusCode::NOT_FOUND` if the order doesn't exist
/// * `StatusCode::INTERNAL_SERVER_ERROR` if database error occurs
pub async fn get_order(
    State(state): State<HandlerState>,
    Path(order_id): Path<String>,
) -> Result<Response<CrossChainOrder>, Response<()>> {
    match state.orderbook.get_order(&order_id).await {
        Ok(Some(order)) => Ok(Response::ok(order)),
        Ok(None) => Err(Response::error("Order not found", StatusCode::NOT_FOUND)),
        Err(e) => {
            tracing::error!("Failed to retrieve order {}: {}", order_id, e);
            Err(Response::error(
                "Failed to retrieve order",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

pub async fn get_orders_by_chain(
    State(state): State<HandlerState>,
    Path(chain_id): Path<u64>,
) -> Result<Response<Vec<CrossChainOrder>>, Response<()>> {
    match state.orderbook.get_orders_by_chain(chain_id).await {
        Ok(orders) => Ok(Response::ok(orders)),
        Err(e) => {
            tracing::error!("Failed to retrieve orders for chain {}: {}", chain_id, e);
            Err(Response::error(
                "Internal error",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

pub async fn submit_secret(
    State(state): State<HandlerState>,
    Json(secret_input): Json<SecretInput>,
) -> Result<StatusCode, Response<()>> {
    // Validate the secret input
    if secret_input.secret.is_empty() {
        return Err(Response::error(
            "Secret cannot be empty",
            StatusCode::BAD_REQUEST,
        ));
    }

    if secret_input.order_hash.is_empty() {
        return Err(Response::error(
            "Order hash cannot be empty",
            StatusCode::BAD_REQUEST,
        ));
    }

    // Validate secret format (should be a hex string without 0x prefix)
    if secret_input.secret.is_empty() || !secret_input.secret.chars().all(|c| c.is_ascii_hexdigit())
    {
        return Err(Response::error(
            "Secret must be a valid hex string without 0x prefix",
            StatusCode::BAD_REQUEST,
        ));
    }

    match state
        .orderbook
        .submit_secret(&secret_input.order_hash, &secret_input.secret)
        .await
    {
        Ok(()) => Ok(StatusCode::ACCEPTED),
        Err(e) => {
            tracing::error!(
                "Failed to submit secret for order {}: {}",
                secret_input.order_hash,
                e
            );
            match e {
                OrderbookError::Validation(msg) => {
                    if msg.contains("Cannot submit secret for order in status") {
                        Err(Response::error(msg, StatusCode::BAD_REQUEST))
                    } else if msg.contains("Order not found") {
                        Err(Response::error("Order not found", StatusCode::NOT_FOUND))
                    } else {
                        Err(Response::error(msg, StatusCode::BAD_REQUEST))
                    }
                }
                _ => Err(Response::error(
                    "Internal server error",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            }
        }
    }
}

pub async fn get_secret(
    State(state): State<HandlerState>,
    Path(order_hash): Path<String>,
) -> Result<Response<SecretResponse>, Response<()>> {
    match state.orderbook.get_secrets(&order_hash).await {
        Ok(secrets) => {
            let secret = secrets.first().cloned();
            Ok(Response::ok(SecretResponse { secret, order_hash }))
        }
        Err(e) => {
            tracing::error!("Failed to retrieve secret for order {}: {}", order_hash, e);
            Err(Response::error(
                "Failed to retrieve secret",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

/// Validate the signed order input
fn validate_signed_order(signed_order: &SignedOrderInput) -> Result<(), String> {
    // Validate signature format (basic check)
    if signed_order.signature.is_null() {
        return Err("Signature cannot be empty".to_string());
    }


    // Validate order data
    validate_order_input(&signed_order.order)?;

    Ok(())
}

/// Validate the order input data
fn validate_order_input(order: &crate::primitives::OrderInput) -> Result<(), String> {
    // Validate salt
    if order.salt.is_empty() {
        return Err("Salt cannot be empty".to_string());
    }

    // Validate addresses (basic format check)
    if !order.maker_asset.starts_with("0x") || order.maker_asset.len() != 42 {
        return Err("Maker asset must be a valid Ethereum address".to_string());
    }

    if !order.taker_asset.starts_with("0x") || order.taker_asset.len() != 42 {
        return Err("Taker asset must be a valid Ethereum address".to_string());
    }

    if !order.maker.starts_with("0x") || order.maker.len() != 42 {
        return Err("Maker must be a valid Ethereum address".to_string());
    }

    if !order.receiver.starts_with("0x") || order.receiver.len() != 42 {
        return Err("Receiver must be a valid Ethereum address".to_string());
    }

    // Validate amounts (basic check for non-empty strings)
    if order.making_amount <= BigDecimal::from(0) {
        return Err("Invalid making amount".to_string());
    }

    if order.taking_amount <= BigDecimal::from(0) {
        return Err("Invalid taking amount".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{OrderStatus, OrderType, SecretEntry};
    use crate::server::Status;
    use axum::http::StatusCode;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// Utility function to directly update order status (for testing only)
    async fn update_order_status(
        orderbook: &OrderbookProvider,
        order_hash: &str,
        status: OrderStatus,
    ) -> Result<(), OrderbookError> {
        sqlx::query("UPDATE orders SET status = $1, updated_at = NOW() WHERE order_hash = $2")
            .bind(status.to_string())
            .bind(order_hash)
            .execute(&orderbook.pool)
            .await?;
        Ok(())
    }

    /// Utility function to generate random hex string
    fn generate_random_hex(length: usize) -> String {
        let mut bytes = vec![0u8; length / 2];
        for byte in &mut bytes {
            *byte = rand::random::<u8>();
        }
        hex::encode(bytes)
    }

    /// Utility function to create a test signed order with random hashes
    fn create_test_signed_order() -> SignedOrderInput {
        let random_salt = generate_random_hex(64);
        let random_secret_hash1 = generate_random_hex(64);
        let random_secret_hash2 = generate_random_hex(64);

        SignedOrderInput {
            order: OrderInput {
                salt: random_salt,
                maker_asset: "0x1234567890123456789012345678901234567890".to_string(),
                taker_asset: "0x0987654321098765432109876543210987654321".to_string(),
                maker: "0x1111111111111111111111111111111111111111".to_string(),
                receiver: "0x2222222222222222222222222222222222222222".to_string(),
                making_amount: BigDecimal::from_str("1000000000000000000").unwrap(), // 1 ETH
                taking_amount: BigDecimal::from_str("2000000000000000000").unwrap(), // 2 ETH
                maker_traits: "0".to_string(),
            },
            src_chain_id: 1,
            dst_chain_id: 137,
            signature: serde_json::json!({
                "r": format!("0x{}", generate_random_hex(128)),
                "vs": format!("0x{}", generate_random_hex(128)),
            }), 
            extension: serde_json::json!({}),
            order_type: OrderType::MultipleFills,
            secrets: Some(vec![
                SecretEntry {
                    index: 0,
                    secret: None,
                    secret_hash: random_secret_hash1,
                },
                SecretEntry {
                    index: 1,
                    secret: None,
                    secret_hash: random_secret_hash2,
                },
            ]),
            deadline: chrono::Utc::now().timestamp_millis() as u64 + 3600000, // 1 hour from now
            taker: "0x3333333333333333333333333333333333333333".to_string(),
            timelock: "0".to_string(),
            taker_traits: "0".to_string(),
            args: serde_json::json!({}),
        }
    }

    /// Test the full flow: submit order -> get active orders -> update status -> test secret submission
    #[tokio::test]
    async fn test_full_order_flow() {
        // Setup test database connection
        let db_url = "postgres://postgres:postgres@localhost:5433/garden";
        let _ = tracing_subscriber::fmt().try_init();
        let orderbook = OrderbookProvider::from_db_url(db_url).await.unwrap();

        let _ = orderbook.drop_tables().await;
        orderbook.create_tables_with_new_schema().await.unwrap();

        let state = HandlerState { orderbook };

        // Test 1: Submit an order
        println!("=== Test 1: Submit Order ===");
        let signed_order = create_test_signed_order();
        let result = submit_order(State(state.clone()), Json(signed_order.clone())).await;
        assert!(result.is_ok(), "Order submission should succeed");
        assert_eq!(result.unwrap(), StatusCode::ACCEPTED);

        // Test 2: Get active orders and verify the order is there
        println!("=== Test 2: Get Active Orders ===");
        let query = ActiveOrdersQuery {
            page: Some(1),
            limit: Some(10),
        };
        let active_orders_result = get_active_orders(State(state.clone()), Query(query)).await;
        dbg!(&active_orders_result);
        assert!(
            active_orders_result.is_ok(),
            "Getting active orders should succeed"
        );

        let active_orders = active_orders_result.unwrap();
        assert_eq!(active_orders.status, Status::Ok);
        assert!(active_orders.result.is_some());

        let orders_data = active_orders.result.unwrap();
        assert_eq!(orders_data.meta.total_items, 1);
        assert_eq!(orders_data.items.len(), 1);

        let order = &orders_data.items[0];
        assert_eq!(order.order_type, OrderType::MultipleFills);
        assert_eq!(order.secrets.len(), 2); // Should have 2 secret entries
        assert_eq!(order.secrets[0].index, 0);
        assert_eq!(order.secrets[1].index, 1);
        assert!(order.secrets[0].secret.is_none()); // Secret should be None initially
        assert!(order.secrets[1].secret.is_none());

        let order_hash = &order.order_hash;
        println!("Order hash: {}", order_hash);

        // Test 3: Get order by hash
        println!("=== Test 3: Get Order by Hash ===");
        let order_result = get_order(State(state.clone()), Path(order_hash.clone())).await;
        assert!(order_result.is_ok(), "Getting order by hash should succeed");

        let order_response = order_result.unwrap();
        assert_eq!(order_response.status, Status::Ok);
        assert!(order_response.result.is_some());

        let order_data = order_response.result.unwrap();
        assert_eq!(order_data.order_hash, *order_hash);
        assert_eq!(order_data.order_type, OrderType::MultipleFills);

        // Test 4: Try to submit secret when order is unmatched (should fail)
        println!("=== Test 4: Submit Secret (Unmatched Status - Should Fail) ===");
        let random_secret1 = generate_random_hex(32);
        let secret_input = SecretInput {
            secret: random_secret1.clone(),
            order_hash: order_hash.clone(),
        };
        let secret_result = submit_secret(State(state.clone()), Json(secret_input)).await;
        assert!(
            secret_result.is_err(),
            "Secret submission should fail for unmatched order"
        );

        let error_response = secret_result.unwrap_err();
        assert_eq!(error_response.status, Status::Error);
        assert!(error_response.error.is_some());
        assert!(
            error_response
                .error
                .unwrap()
                .contains("Cannot submit secret for order in status")
        );

        // Test 5: Update order status to source_filled
        println!("=== Test 5: Update Status to Source Filled ===");
        update_order_status(&state.orderbook, order_hash, OrderStatus::SourceFilled)
            .await
            .unwrap();

        // Test 6: Try to submit secret when order is source_filled (should fail)
        println!("=== Test 6: Submit Secret (Source Filled Status - Should Fail) ===");
        let secret_input = SecretInput {
            secret: random_secret1.clone(),
            order_hash: order_hash.clone(),
        };
        let secret_result = submit_secret(State(state.clone()), Json(secret_input)).await;
        assert!(
            secret_result.is_err(),
            "Secret submission should fail for source_filled order"
        );

        // Test 7: Update order status to finality_confirmed
        println!("=== Test 7: Update Status to Finality Confirmed ===");
        update_order_status(&state.orderbook, order_hash, OrderStatus::FinalityConfirmed)
            .await
            .unwrap();

        // Test 8: Submit secret when order is finality_confirmed (should succeed)
        println!("=== Test 8: Submit Secret (Finality Confirmed Status - Should Succeed) ===");
        let secret_input = SecretInput {
            secret: random_secret1.clone(),
            order_hash: order_hash.clone(),
        };
        let secret_result = submit_secret(State(state.clone()), Json(secret_input)).await;
        assert!(
            secret_result.is_ok(),
            "Secret submission should succeed for finality_confirmed order"
        );
        assert_eq!(secret_result.unwrap(), StatusCode::ACCEPTED);

        // Test 9: Get secrets and verify the secret was stored
        println!("=== Test 9: Get Secrets ===");
        let secrets_result = get_secret(State(state.clone()), Path(order_hash.clone())).await;
        assert!(secrets_result.is_ok(), "Getting secrets should succeed");

        let secrets_response = secrets_result.unwrap();
        assert_eq!(secrets_response.status, Status::Ok);
        assert!(secrets_response.result.is_some());

        let secret_data = secrets_response.result.unwrap();
        assert_eq!(secret_data.order_hash, *order_hash);
        assert!(secret_data.secret.is_some());
        assert_eq!(secret_data.secret.unwrap(), random_secret1);

        // Test 10: Submit another secret (should succeed)
        println!("=== Test 10: Submit Second Secret ===");
        let random_secret2 = generate_random_hex(32);
        let secret_input = SecretInput {
            secret: random_secret2.clone(),
            order_hash: order_hash.clone(),
        };
        let secret_result = submit_secret(State(state.clone()), Json(secret_input)).await;
        assert!(
            secret_result.is_ok(),
            "Second secret submission should succeed"
        );

        // Test 11: Get secrets and verify both secrets were stored
        println!("=== Test 11: Get All Secrets ===");
        let secrets_result = get_secret(State(state.clone()), Path(order_hash.clone())).await;
        assert!(secrets_result.is_ok(), "Getting all secrets should succeed");

        let secrets_response = secrets_result.unwrap();
        let secret_data = secrets_response.result.unwrap();
        assert_eq!(secret_data.order_hash, *order_hash);
        assert!(secret_data.secret.is_some());
        // Should return the first secret (as per current implementation)
        assert_eq!(secret_data.secret.unwrap(), random_secret1);

        // Test 12: Update order status to source_settled and verify it's no longer active
        println!("=== Test 12: Update Status to Source Settled ===");
        update_order_status(&state.orderbook, order_hash, OrderStatus::SourceSettled)
            .await
            .unwrap();

        let query = ActiveOrdersQuery {
            page: Some(1),
            limit: Some(10),
        };
        let active_orders_result = get_active_orders(State(state.clone()), Query(query)).await;
        assert!(
            active_orders_result.is_ok(),
            "Getting active orders should succeed"
        );

        let active_orders = active_orders_result.unwrap();
        let orders_data = active_orders.result.unwrap();
        assert_eq!(
            orders_data.meta.total_items, 0,
            "Order should no longer be active"
        );
        assert_eq!(
            orders_data.items.len(),
            0,
            "Order should no longer be in active list"
        );

        // Test 13: Get order by hash should still work
        println!("=== Test 13: Get Settled Order by Hash ===");
        let order_result = get_order(State(state.clone()), Path(order_hash.clone())).await;
        assert!(
            order_result.is_ok(),
            "Getting settled order by hash should succeed"
        );

        let order_response = order_result.unwrap();
        let order_data = order_response.result.unwrap();
        assert_eq!(order_data.order_hash, *order_hash);
        assert_eq!(order_data.status.to_string(), "source_settled");
    }

    /// Test order submission with different order types
    #[tokio::test]
    async fn test_order_types() {
        let db_url = "postgres://postgres:postgres@localhost:5433/garden";
        let orderbook = OrderbookProvider::from_db_url(db_url).await.unwrap();

        // Drop tables and recreate to avoid type conflicts
        let _ = orderbook.drop_tables().await;
        orderbook.create_tables_with_new_schema().await.unwrap();

        let state = HandlerState { orderbook };

        // Test SingleFill order
        let mut single_fill_order = create_test_signed_order();
        single_fill_order.order_type = OrderType::SingleFill;
        single_fill_order.secrets = None; // Single fill orders don't need secrets

        let result = submit_order(State(state.clone()), Json(single_fill_order)).await;
        assert!(
            result.is_ok(),
            "Single fill order submission should succeed"
        );

        // Test MultipleFills order
        let multiple_fills_order = create_test_signed_order();
        let result = submit_order(State(state.clone()), Json(multiple_fills_order)).await;
        assert!(
            result.is_ok(),
            "Multiple fills order submission should succeed"
        );

        // Verify both orders are active
        let query = ActiveOrdersQuery {
            page: Some(1),
            limit: Some(10),
        };
        let active_orders_result = get_active_orders(State(state.clone()), Query(query)).await;
        assert!(active_orders_result.is_ok());

        let active_orders = active_orders_result.unwrap();
        let orders_data = active_orders.result.unwrap();
        assert_eq!(orders_data.meta.total_items, 2);
        assert_eq!(orders_data.items.len(), 2);
    }

    /// Test validation errors
    #[tokio::test]
    async fn test_validation_errors() {
        let db_url = "postgres://postgres:postgres@localhost:5433/garden";
        let orderbook = OrderbookProvider::from_db_url(db_url).await.unwrap();

        // Drop tables and recreate to avoid type conflicts
        let _ = orderbook.drop_tables().await;
        orderbook.create_tables_with_new_schema().await.unwrap();

        let state = HandlerState { orderbook };

        // Test invalid secret format
        let invalid_secret_input = SecretInput {
            secret: "invalid_secret_with_0x_prefix".to_string(),
            order_hash: generate_random_hex(64),
        };

        let result = submit_secret(State(state.clone()), Json(invalid_secret_input)).await;
        assert!(result.is_err());
        let error_response = result.unwrap_err();
        assert!(
            error_response
                .error
                .unwrap()
                .contains("Secret must be a valid hex string")
        );

        // Test empty secret
        let empty_secret_input = SecretInput {
            secret: "".to_string(),
            order_hash: generate_random_hex(64),
        };

        let result = submit_secret(State(state.clone()), Json(empty_secret_input)).await;
        assert!(result.is_err());
        let error_response = result.unwrap_err();
        assert!(
            error_response
                .error
                .unwrap()
                .contains("Secret cannot be empty")
        );

        // Test non-existent order
        let non_existent_input = SecretInput {
            secret: generate_random_hex(32),
            order_hash: generate_random_hex(64),
        };

        let result = submit_secret(State(state.clone()), Json(non_existent_input)).await;
        assert!(result.is_err());
        let error_response = result.unwrap_err();
        assert!(error_response.error.unwrap().contains("Order not found"));
    }
}
