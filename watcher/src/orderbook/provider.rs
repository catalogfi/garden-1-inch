use crate::{
    orderbook::errors::OrderbookError,
    types::{OrderStatus, WatcherEventType},
};
use alloy::rpc::types::Log;
use sqlx::Row;
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct OrderbookProvider {
    pub pool: Pool<Postgres>,
}

#[derive(Debug, Clone)]
pub struct PendingOrder {
    pub order_hash: String,
    pub src_escrow_address: Option<String>,
    pub dst_escrow_address: Option<String>,
    pub src_chain_id: String,
    pub dst_chain_id: String,
    pub status: String,
}

pub struct OrderEscrowInfo {
    pub status: String,
    pub src_escrow: Option<String>,
    pub dst_escrow: Option<String>,
}

impl OrderbookProvider {
    pub fn new(pool: Pool<Postgres>) -> Self {
        OrderbookProvider { pool }
    }

    pub fn normalize_order_hash(&self, order_hash: &str) -> String {
        if order_hash.starts_with("0x") {
            order_hash.to_lowercase()
        } else {
            format!("0x{}", order_hash.to_lowercase())
        }
    }

    pub fn normalize_address(&self, address: &str) -> String {
        if address.starts_with("0x") {
            address.to_lowercase()
        } else {
            format!("0x{}", address.to_lowercase())
        }
    }
    pub async fn get_order_status(&self, order_hash: &str) -> Result<String, OrderbookError> {
        let query = "SELECT status FROM orders WHERE order_hash = $1";
        sqlx::query_scalar(query)
            .bind(self.normalize_order_hash(order_hash))
            .fetch_one(&self.pool)
            .await
            .map_err(OrderbookError::from)
    }

    pub async fn is_escrow_source(
        &self,
        order_hash: &str,
        escrow_addr: &str,
    ) -> Result<bool, OrderbookError> {
        let query = "SELECT src_escrow_address = $1 FROM orders WHERE order_hash = $2";
        sqlx::query_scalar(query)
            .bind(self.normalize_address(escrow_addr))
            .bind(self.normalize_order_hash(order_hash))
            .fetch_one(&self.pool)
            .await
            .map_err(OrderbookError::from)
    }

    pub async fn is_escrow_destination(
        &self,
        order_hash: &str,
        escrow_addr: &str,
    ) -> Result<bool, OrderbookError> {
        let query = "SELECT dst_escrow_address = $1 FROM orders WHERE order_hash = $2";
        sqlx::query_scalar(query)
            .bind(self.normalize_address(escrow_addr))
            .bind(self.normalize_order_hash(order_hash))
            .fetch_one(&self.pool)
            .await
            .map_err(OrderbookError::from)
    }
    pub async fn from_db_url(db_url: &str) -> Result<Self, OrderbookError> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2000)
            .connect(db_url)
            .await?;
        Ok(Self::new(pool))
    }

    /// Table Schema for cross_chain_orders
    /// This table will store all cross-chain orders with their statuses
    pub async fn create_tables(&self) -> Result<(), OrderbookError> {
        let create_table_sql = r#"
            CREATE TABLE IF NOT EXISTS orders (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                order_hash VARCHAR(66) UNIQUE NOT NULL,
                quote_id VARCHAR(255) NOT NULL,
                src_chain_id VARCHAR(256) NOT NULL,
                dst_chain_id VARCHAR(256) NOT NULL,
                maker VARCHAR(42) NOT NULL,
                receiver VARCHAR(42) NOT NULL,
                maker_asset VARCHAR(42) NOT NULL,
                taker_asset VARCHAR(42) NOT NULL,
                making_amount NUMERIC NOT NULL,
                taking_amount NUMERIC NOT NULL,
                salt VARCHAR(255) NOT NULL,
                maker_traits VARCHAR(255) NOT NULL DEFAULT '0',
                signature TEXT NOT NULL,
                extension TEXT NOT NULL,
                order_type TEXT NOT NULL DEFAULT 'single_fill',
                secrets JSONB NOT NULL DEFAULT '[]'::jsonb,
                source_immutables JSONB DEFAULT '[]'::jsonb,
                src_event JSONB,
                dest_event JSONB,
                
                -- Status and lifecycle fields
                status TEXT NOT NULL DEFAULT 'unmatched',
                deadline BIGINT NOT NULL,
                auction_start_date BIGINT,
                auction_end_date BIGINT,

                -- Fill tracking
                src_escrow_address VARCHAR(42),
                dst_escrow_address VARCHAR(42),
                src_tx_hash VARCHAR(66),
                dst_tx_hash VARCHAR(66),
                filled_maker_amount NUMERIC DEFAULT 0,
                filled_taker_amount NUMERIC DEFAULT 0,
                
                
                -- Timestamps
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
        "#;

        sqlx::query(create_table_sql).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn handle_escrow_event(
        &self,
        order_hash: &str,
        event_type: WatcherEventType,
        escrow_address: &str,
        block_hash: &str,
        log: Log,
    ) -> Result<(), OrderbookError> {
        let normalized_order_hash = self.normalize_order_hash(order_hash);
        let normalized_escrow = self.normalize_address(escrow_address);

        // Get current status first for withdrawal events
        let current_status = if matches!(
            event_type,
            WatcherEventType::SourceWithdraw | WatcherEventType::DestinationWithdraw
        ) {
            Some(self.get_order_status(order_hash).await?)
        } else {
            None
        };

        let (status, address_field, tx_hash_field, log_field, should_update_log) = match event_type
        {
            WatcherEventType::SrcEscrowCreatedEvent => (
                OrderStatus::SourceFilled,
                "src_escrow_address",
                "src_tx_hash",
                "src_event",
                true,
            ),
            WatcherEventType::DstEscrowCreatedEvent => (
                OrderStatus::DestinationFilled,
                "dst_escrow_address",
                "dst_tx_hash",
                "dest_event",
                true,
            ),
            WatcherEventType::SourceWithdraw => {
                // Check if destination is already settled
                if let Some(ref current) = current_status {
                    if current == "destination_settled" {
                        (
                            OrderStatus::FulFilled,
                            "src_escrow_address",
                            "src_tx_hash",
                            "",
                            false,
                        )
                    } else {
                        (
                            OrderStatus::SourceSettled,
                            "src_escrow_address",
                            "src_tx_hash",
                            "",
                            false,
                        )
                    }
                } else {
                    (
                        OrderStatus::SourceSettled,
                        "src_escrow_address",
                        "src_tx_hash",
                        "",
                        false,
                    )
                }
            }
            WatcherEventType::DestinationWithdraw => {
                // Check if source is already settled
                if let Some(ref current) = current_status {
                    if current == "source_settled" {
                        (
                            OrderStatus::FulFilled,
                            "dst_escrow_address",
                            "dst_tx_hash",
                            "",
                            false,
                        )
                    } else {
                        (
                            OrderStatus::DestinationSettled,
                            "dst_escrow_address",
                            "dst_tx_hash",
                            "",
                            false,
                        )
                    }
                } else {
                    (
                        OrderStatus::DestinationSettled,
                        "dst_escrow_address",
                        "dst_tx_hash",
                        "",
                        false,
                    )
                }
            }
            WatcherEventType::SourceRescue => (
                OrderStatus::SourceRefunded,
                "src_escrow_address",
                "src_tx_hash",
                "",
                false,
            ),
            WatcherEventType::DestinationRescue => (
                OrderStatus::DestinationRefunded,
                "dst_escrow_address",
                "dst_tx_hash",
                "",
                false,
            ),
        };

        let query = if should_update_log {
            format!(
                r#"
            UPDATE orders 
            SET status = $1, {address_field} = $2, {tx_hash_field} = $4, {log_field} = $5, updated_at = NOW()
            WHERE order_hash = $3
            "#,
            )
        } else {
            format!(
                r#"
            UPDATE orders 
            SET status = $1, {address_field} = $2, {tx_hash_field} = $4, updated_at = NOW()
            WHERE order_hash = $3
            "#,
            )
        };

        let result = if should_update_log {
            let log_json = serde_json::to_value(&log).unwrap_or(serde_json::Value::Null);

            sqlx::query(&query)
                .bind(status.to_string())
                .bind(normalized_escrow)
                .bind(normalized_order_hash)
                .bind(block_hash)
                .bind(log_json)
                .execute(&self.pool)
                .await?
        } else {
            sqlx::query(&query)
                .bind(status.to_string())
                .bind(normalized_escrow)
                .bind(normalized_order_hash)
                .bind(block_hash)
                .execute(&self.pool)
                .await?
        };

        if result.rows_affected() == 0 {
            // tracing::warn!(
            //     "No rows updated for order_hash: {} - order may not exist in database",
            //     order_hash
            // );
        } else {
            tracing::info!(
                "💾 Successfully updated order {} with status {} and escrow address {} and tx hash {}",
                order_hash,
                status.to_string(),
                escrow_address,
                block_hash
            );

            // Log if order was marked as fulfilled
            if status.to_string() == "fulfilled" {
                tracing::info!(
                    "Order {} has been marked as FULFILLED due to both settlements complete",
                    order_hash
                );
            }
        }

        Ok(())
    }

    pub async fn update_order_status(
        &self,
        order_hash: &str,
        status: OrderStatus,
    ) -> Result<(), OrderbookError> {
        let normalized_order_hash = self.normalize_order_hash(order_hash);
        let query = r#"
            UPDATE orders 
            SET status = $1, updated_at = NOW()
            WHERE order_hash = $2
        "#;

        let result = sqlx::query(query)
            .bind(status.to_string())
            .bind(normalized_order_hash)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            // tracing::warn!(
            //     "No rows updated for order_hash: {} - order may not exist in database",
            //     order_hash
            // );
        } else {
            tracing::info!(
                "💾 Successfully updated order {} with status {}",
                order_hash,
                status.to_string(),
            );
        }

        Ok(())
    }

    pub async fn determine_withdrawal_status(
        &self,
        order_hash: &str,
        escrow_address: &str,
    ) -> anyhow::Result<OrderStatus> {
        let normalized_order_hash = self.normalize_order_hash(order_hash);
        let normalized_escrow = self.normalize_address(escrow_address);

        let query = r#"
            SELECT src_escrow_address, dst_escrow_address
            FROM orders 
            WHERE order_hash = $1
        "#;

        let row = sqlx::query(query)
            .bind(normalized_order_hash)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database query failed: {}", e))?;

        let row = row.ok_or_else(|| anyhow::anyhow!("Order not found: {}", order_hash))?;

        let src_escrow: Option<String> = row.try_get("src_escrow_address").ok();
        let dst_escrow: Option<String> = row.try_get("dst_escrow_address").ok();

        if let Some(src_addr) = src_escrow {
            if self.normalize_address(&src_addr) == normalized_escrow {
                return Ok(OrderStatus::SourceSettled);
            }
        }

        if let Some(dst_addr) = dst_escrow {
            if dst_addr.to_lowercase() == escrow_address.to_lowercase() {
                return Ok(OrderStatus::DestinationSettled);
            }
        }

        Err(anyhow::anyhow!(
            "Could not determine withdrawal type for order {} with escrow address {}",
            order_hash,
            escrow_address
        ))
    }

    /// Fetch all pending orders that have escrow addresses but are not yet settled/completed
    pub async fn fetch_pending_orders(&self) -> Result<Vec<PendingOrder>, OrderbookError> {
        let query = r#"
            SELECT 
                order_hash,
                src_escrow_address,
                dst_escrow_address,
                src_chain_id,
                dst_chain_id,
                status
            FROM orders 
            WHERE status NOT IN ('unmatched', 'expired', 'source_settled', 'destination_settled', 'cancelled')
            AND (src_escrow_address IS NOT NULL OR dst_escrow_address IS NOT NULL)
            AND deadline > EXTRACT(EPOCH FROM NOW())
        "#;

        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        let orders = rows
            .into_iter()
            .map(|row| PendingOrder {
                order_hash: row.get("order_hash"),
                src_escrow_address: row.get("src_escrow_address"),
                dst_escrow_address: row.get("dst_escrow_address"),
                src_chain_id: row.get("src_chain_id"),
                dst_chain_id: row.get("dst_chain_id"),
                status: row.get("status"),
            })
            .collect();

        Ok(orders)
    }

    /// Get escrow addresses grouped by chain for efficient monitoring
    pub async fn get_escrow_addresses_by_chain(
        &self,
    ) -> Result<std::collections::HashMap<String, Vec<String>>, OrderbookError> {
        let query = r#"
        SELECT DISTINCT src_chain_id as chain_id, src_escrow_address as escrow_address
        FROM orders 
        WHERE src_escrow_address IS NOT NULL 
        AND status NOT IN ('unmatched', 'expired', 'source_settled', 'destination_settled', 'cancelled')
        AND deadline > EXTRACT(EPOCH FROM NOW())
        
        UNION
        
        SELECT DISTINCT dst_chain_id as chain_id, dst_escrow_address as escrow_address
        FROM orders 
        WHERE dst_escrow_address IS NOT NULL 
        AND status NOT IN ('unmatched', 'expired', 'source_settled', 'destination_settled', 'cancelled')
        AND deadline > EXTRACT(EPOCH FROM NOW())
    "#;

        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        let mut chain_escrows = std::collections::HashMap::new();

        for row in rows {
            let chain_id: String = row.get("chain_id");
            let escrow_address: String = row.get("escrow_address");

            chain_escrows
                .entry(chain_id)
                .or_insert_with(Vec::new)
                .push(escrow_address);
        }

        Ok(chain_escrows)
    }

    pub async fn get_escrow_addresses_with_order_hashes_by_chain(
        &self,
    ) -> Result<std::collections::HashMap<String, Vec<(String, String)>>, OrderbookError> {
        let query = r#"
        SELECT src_chain_id as chain_id, 
               LOWER(src_escrow_address) as escrow_address, 
               LOWER(order_hash) as order_hash
        FROM orders 
        WHERE src_escrow_address IS NOT NULL 
        AND (
            (status IN ('source_filled', 'destination_filled')) OR
            (status = 'source_settled' AND dst_escrow_address IS NOT NULL) OR
            (status = 'destination_settled' AND src_escrow_address IS NOT NULL)
        )
        AND deadline > EXTRACT(EPOCH FROM NOW())

        UNION

        SELECT dst_chain_id as chain_id, 
               LOWER(dst_escrow_address) as escrow_address, 
               LOWER(order_hash) as order_hash
        FROM orders 
        WHERE dst_escrow_address IS NOT NULL 
        AND (
            (status IN ('source_filled', 'destination_filled')) OR
            (status = 'destination_settled' AND src_escrow_address IS NOT NULL) OR
            (status = 'source_settled' AND dst_escrow_address IS NOT NULL)
        )
        AND deadline > EXTRACT(EPOCH FROM NOW())
    "#;

        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        let mut chain_escrows = std::collections::HashMap::new();

        for row in rows {
            let chain_id: String = row.get("chain_id");
            let escrow_address: String = row.get("escrow_address");
            let order_hash: String = row.get("order_hash");

            chain_escrows
                .entry(chain_id)
                .or_insert_with(Vec::new)
                .push((escrow_address, order_hash));
        }

        Ok(chain_escrows)
    }

    pub async fn get_order_status_and_escrows(
        &self,
        order_hash: &str,
    ) -> Result<OrderEscrowInfo, OrderbookError> {
        let query = r#"
        SELECT status, src_escrow_address, dst_escrow_address
        FROM orders 
        WHERE order_hash = $1
    "#;

        let row = sqlx::query(query)
            .bind(self.normalize_order_hash(order_hash))
            .fetch_one(&self.pool)
            .await?;

        Ok(OrderEscrowInfo {
            status: row.get("status"),
            src_escrow: row.get("src_escrow_address"),
            dst_escrow: row.get("dst_escrow_address"),
        })
    }

    /// Check if an order is completely settled (both source and destination withdrawn)
    pub async fn is_order_complete(&self, order_hash: &str) -> Result<bool, OrderbookError> {
        let query = r#"
            SELECT status
            FROM orders 
            WHERE order_hash = $1
        "#;

        let row = sqlx::query(query)
            .bind(format!("0x{}", order_hash.trim_start_matches("0x")))
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let status: String = row.get("status");
            // Order is complete if both source and destination are settled
            Ok(status == "source_settled" || status == "destination_settled")
        } else {
            Ok(false) // Order not found, consider it not complete
        }
    }
    pub async fn is_order_fully_complete(&self, order_hash: &str) -> Result<bool, OrderbookError> {
        let query = r#"
        SELECT 
            status,
            src_escrow_address IS NOT NULL as has_src_escrow,
            dst_escrow_address IS NOT NULL as has_dst_escrow,
            (SELECT COUNT(*) FROM (
                SELECT 1 WHERE src_escrow_address IS NOT NULL
                UNION ALL
                SELECT 1 WHERE dst_escrow_address IS NOT NULL
            ) as escrow_count) as total_escrows
        FROM orders 
        WHERE order_hash = $1
    "#;

        let row = sqlx::query(query)
            .bind(self.normalize_order_hash(order_hash))
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => {
                let status: String = row.get("status");
                let has_src_escrow: bool = row.get("has_src_escrow");
                let has_dst_escrow: bool = row.get("has_dst_escrow");

                // If already fulfilled, return true
                if status == "fulfilled" {
                    return Ok(true);
                }

                // Check if both escrows are settled
                let src_settled = !has_src_escrow || status == "source_settled";
                let dst_settled = !has_dst_escrow || status == "destination_settled";

                Ok(src_settled && dst_settled && (has_src_escrow || has_dst_escrow))
            }
            None => Ok(false), // Order not found
        }
    }

    pub async fn check_and_update_fulfilled_status(
        &self,
        order_hash: &str,
    ) -> Result<bool, OrderbookError> {
        let query = r#"
        SELECT 
            status,
            src_escrow_address,
            dst_escrow_address
        FROM orders 
        WHERE order_hash = $1
    "#;

        let row = sqlx::query(query)
            .bind(self.normalize_order_hash(order_hash))
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => {
                let current_status: String = row.get("status");
                let src_escrow: Option<String> = row.get("src_escrow_address");
                let dst_escrow: Option<String> = row.get("dst_escrow_address");

                // If already fulfilled, return true
                if current_status == "fulfilled" {
                    return Ok(true);
                }

                // Check if both escrows exist and are settled
                let both_settled = match (src_escrow.is_some(), dst_escrow.is_some()) {
                    (true, true) => {
                        // Both escrows exist, check if both are settled
                        current_status == "source_settled"
                            || current_status == "destination_settled"
                    }
                    (true, false) => current_status == "source_settled",
                    (false, true) => current_status == "destination_settled",
                    (false, false) => false, // No escrows, can't be fulfilled
                };

                if both_settled {
                    // Update to fulfilled status
                    self.update_order_status(order_hash, OrderStatus::FulFilled)
                        .await?;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Ok(false), // Order not found
        }
    }
}
