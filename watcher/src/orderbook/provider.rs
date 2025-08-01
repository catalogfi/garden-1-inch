use crate::{
    orderbook::errors::OrderbookError,
    types::{OrderStatus, WatcherEventType},
};
use sqlx::{Pool, Postgres};

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
            CREATE TABLE cross_chain_orders (
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
    ) -> Result<(), OrderbookError> {
        let (status, address_field) = match event_type {
            WatcherEventType::SourceEscrowCreated => (OrderStatus::Unmatched, "src_escrow_address"),
            WatcherEventType::SourceEscrowUpdated => {
                (OrderStatus::SourceFilled, "src_escrow_address")
            }
            WatcherEventType::SourceEscrowClosed => {
                (OrderStatus::SourceSettled, "src_escrow_address")
            }
            WatcherEventType::DestinationEscrowCreated => {
                (OrderStatus::SourceWithdrawPending, "dst_escrow_address")
            }
            WatcherEventType::DestinationEscrowUpdated => {
                (OrderStatus::DestinationFilled, "dst_escrow_address")
            }
            WatcherEventType::DestinationEscrowClosed => {
                (OrderStatus::DestinationSettled, "dst_escrow_address")
            }
        };

        let query = format!(
            r#"
            UPDATE cross_chain_orders 
            SET status = $1, {address_field} = $2, updated_at = NOW()
            WHERE order_hash = $3
        "#
        );

        sqlx::query(&query)
            .bind(status.to_string())
            .bind(escrow_address)
            .bind(order_hash)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
