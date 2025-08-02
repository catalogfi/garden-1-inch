use crate::{
    orderbook::errors::OrderbookError,
    types::{OrderStatus, SecretEntry, WatcherEventType},
};
use serde_json::Value;
use sqlx::{Pool, Postgres};
use tracing::warn;

#[derive(Clone, Debug)]
pub struct OrderbookProvider {
    pub pool: Pool<Postgres>,
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
                bitcoin_address VARCHAR(66),
                
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
    ) -> Result<(), OrderbookError> {
        let (status, address_field, tx_hash_field) = match event_type {
            WatcherEventType::SrcEscrowCreatedEvent => (
                OrderStatus::SourceFilled,
                "src_escrow_address",
                "src_tx_hash",
            ),
            WatcherEventType::DstEscrowCreatedEvent => (
                OrderStatus::DestinationFilled,
                "dst_escrow_address",
                "dst_tx_hash",
            ),
            WatcherEventType::SourceWithdraw => (
                OrderStatus::SourceSettled,
                "src_escrow_address",
                "src_tx_hash",
            ),
            WatcherEventType::DestinationWithdraw => (
                OrderStatus::DestinationSettled,
                "dst_escrow_address",
                "dst_tx_hash",
            ),
            WatcherEventType::SourceRescue => (
                OrderStatus::SourceRefunded,
                "src_escrow_address",
                "src_tx_hash",
            ),
            WatcherEventType::DestinationRescue => (
                OrderStatus::DestinationRefunded,
                "dst_escrow_address",
                "dst_tx_hash",
            ),
        };

        let query = format!(
            r#"
        UPDATE orders 
        SET status = $1, {address_field} = $2, {tx_hash_field} = $4, updated_at = NOW()
        WHERE order_hash = $3
        "#,
        );

        let result = sqlx::query(&query)
            .bind(status.to_string())
            .bind(escrow_address)
            .bind(format!("0x{order_hash}"))
            .bind(block_hash)
            .execute(&self.pool)
            .await?;

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

    pub async fn update_secrets(
        &self,
        order_hash: &str,
        secrets: &Value,
    ) -> Result<(), OrderbookError> {
        let parsed: Vec<SecretEntry> = serde_json::from_value(secrets.clone()).map_err(|e| {
            warn!("Failed to deserialize secrets: {}", e);
            OrderbookError::Serialization(e.to_string())
        })?;

        let result = sqlx::query(
            r#"
        UPDATE orders
        SET secrets = $1, updated_at = NOW()
        WHERE order_hash = $2
        "#,
        )
        .bind(secrets)
        .bind(order_hash)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            tracing::warn!(
                "No rows updated when setting secrets for order_hash: {}",
                order_hash
            );
            return Err(OrderbookError::NotFound(format!(
                "Order with hash {order_hash} not found"
            )));
        } else {
            tracing::info!(
                "Secrets updated for order_hash: {} with {} entries",
                order_hash,
                parsed.len()
            );
        }

        Ok(())
    }
}
