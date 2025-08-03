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
    pub src_chain_id: i64,
    pub dst_chain_id: i64,
    pub status: String,
}

impl OrderbookProvider {
    pub fn new(pool: Pool<Postgres>) -> Self {
        OrderbookProvider { pool }
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
                src_chain_id BIGINT NOT NULL,
                dst_chain_id BIGINT NOT NULL,
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
            WatcherEventType::SourceWithdraw => (
                OrderStatus::SourceSettled,
                "src_escrow_address",
                "src_tx_hash",
                "",
                false,
            ),
            WatcherEventType::DestinationWithdraw => (
                OrderStatus::DestinationSettled,
                "dst_escrow_address",
                "dst_tx_hash",
                "",
                false,
            ),
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
                .bind(escrow_address)
                .bind(format!("0x{order_hash}"))
                .bind(block_hash)
                .bind(log_json)
                .execute(&self.pool)
                .await?
        } else {
            sqlx::query(&query)
                .bind(status.to_string())
                .bind(escrow_address)
                .bind(format!("0x{order_hash}"))
                .bind(block_hash)
                .execute(&self.pool)
                .await?
        };

        if result.rows_affected() == 0 {
            tracing::warn!(
                "No rows updated for order_hash: {} - order may not exist in database",
                order_hash
            );
        } else {
            tracing::info!(
                "Successfully updated order {} with status {} and escrow address {} and tx hash {}",
                order_hash,
                status.to_string(),
                escrow_address,
                block_hash
            );
        }

        Ok(())
    }
    pub async fn update_order_status(
        &self,
        order_hash: &str,
        status: OrderStatus,
    ) -> Result<(), OrderbookError> {
        let query = r#"
            UPDATE orders 
            SET status = $1, updated_at = NOW()
            WHERE order_hash = $2
        "#;

        let result = sqlx::query(query)
            .bind(status.to_string())
            .bind(format!("0x{order_hash}"))
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            tracing::warn!(
                "No rows updated for order_hash: {} - order may not exist in database",
                order_hash
            );
        } else {
            tracing::info!(
                "Successfully updated order {} with status {}",
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
        let query = r#"
            SELECT src_escrow_address, dst_escrow_address
            FROM orders 
            WHERE order_hash = $1
        "#;

        let row = sqlx::query(query)
            .bind(format!("0x{order_hash}"))
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database query failed: {}", e))?;

        let row = row.ok_or_else(|| anyhow::anyhow!("Order not found: {}", order_hash))?;

        let src_escrow: Option<String> = row.try_get("src_escrow_address").ok();
        let dst_escrow: Option<String> = row.try_get("dst_escrow_address").ok();

        if let Some(src_addr) = src_escrow {
            if src_addr.to_lowercase() == escrow_address.to_lowercase() {
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
    ) -> Result<std::collections::HashMap<i64, Vec<String>>, OrderbookError> {
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
            let chain_id: i64 = row.get("chain_id");
            let escrow_address: String = row.get("escrow_address");

            chain_escrows
                .entry(chain_id)
                .or_insert_with(Vec::new)
                .push(escrow_address);
        }

        Ok(chain_escrows)
    }
}
