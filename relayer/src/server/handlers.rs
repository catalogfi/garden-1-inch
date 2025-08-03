use crate::{
    orderbook::{OrderbookProvider, provider::OrderbookError},
    primitives::{
        ActiveOrderOutput, CrossChainOrder, GetActiveOrdersOutput, Meta, OrderInput, SecretInput,
        SecretResponse, SignedOrderInput, UpdateOrderFieldRequest,
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
                        src_deploy_immutables: order.src_deploy_immutables.clone(),
                        dst_deploy_immutables: order.dst_deploy_immutables.clone(),
                        src_withdraw_immutables: order.src_withdraw_immutables.clone(),
                        dst_withdraw_immutables: order.dst_withdraw_immutables.clone(),
                        src_event: order.src_event.clone(),
                        dest_event: order.dest_event.clone(),
                        src_withdraw: order.src_withdraw.clone(),
                        dst_withdraw: order.dst_withdraw.clone(),
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

/// Update a specific field for an order
///
/// This endpoint allows updating specific JSONB fields for an existing order.
///
/// # Arguments
/// * `State(state)` - The handler state
/// * `Path(order_hash)` - The order hash to update
/// * `Json(update_request)` - The update request containing field name and JSON value
///
/// # Returns
/// * `StatusCode::OK` if the field was updated successfully
/// * `StatusCode::BAD_REQUEST` if validation fails
/// * `StatusCode::NOT_FOUND` if the order doesn't exist
/// * `StatusCode::INTERNAL_SERVER_ERROR` if database error occurs
pub async fn update_order_field(
    State(state): State<HandlerState>,
    Path(order_hash): Path<String>,
    Json(update_request): Json<UpdateOrderFieldRequest>,
) -> Result<StatusCode, Response<()>> {
    // Validate the update request
    if update_request.field_name.is_empty() {
        return Err(Response::error(
            "Field name cannot be empty",
            StatusCode::BAD_REQUEST,
        ));
    }

    if update_request.order_hash.is_empty() {
        return Err(Response::error(
            "Order hash cannot be empty",
            StatusCode::BAD_REQUEST,
        ));
    }

    // Ensure the order_hash in the path matches the one in the request
    if update_request.order_hash != order_hash {
        return Err(Response::error(
            "Order hash in path does not match order hash in request body",
            StatusCode::BAD_REQUEST,
        ));
    }

    match state
        .orderbook
        .update_order_field(&update_request.order_hash, &update_request.field_name, &update_request.value)
        .await
    {
        Ok(()) => Ok(StatusCode::OK),
        Err(e) => {
            tracing::error!(
                "Failed to update field {} for order {}: {}",
                update_request.field_name,
                update_request.order_hash,
                e
            );
            match e {
                OrderbookError::Validation(msg) => {
                    if msg.contains("Order not found") {
                        Err(Response::error("Order not found", StatusCode::NOT_FOUND))
                    } else if msg.contains("Invalid field name") {
                        Err(Response::error(msg, StatusCode::BAD_REQUEST))
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