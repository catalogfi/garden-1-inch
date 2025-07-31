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

    /// Create the orders table with updated schema
    pub async fn create_tables(&self) -> Result<(), OrderbookError> {
        let create_table_sql = r#"
            CREATE TABLE IF NOT EXISTS cross_chain_orders (
                id UUID PRIMARY KEY,
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
                secret_hashes JSONB,
                secrets JSONB,
                status VARCHAR(50) NOT NULL DEFAULT 'unmatched',
                deadline BIGINT,
                auction_start_date TIMESTAMP WITH TIME ZONE,
                auction_end_date TIMESTAMP WITH TIME ZONE,
                src_escrow_address VARCHAR(42),
                dst_escrow_address VARCHAR(42),
                src_tx_hash VARCHAR(66),
                dst_tx_hash VARCHAR(66),
                filled_maker_amount NUMERIC DEFAULT 0,
                filled_taker_amount NUMERIC DEFAULT 0,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL
            )
        "#;

        sqlx::query(create_table_sql).execute(&self.pool).await?;

        // Create indexes
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_orders_maker ON cross_chain_orders(maker)",
            "CREATE INDEX IF NOT EXISTS idx_orders_src_chain ON cross_chain_orders(src_chain_id)",
            "CREATE INDEX IF NOT EXISTS idx_orders_dst_chain ON cross_chain_orders(dst_chain_id)",
            "CREATE INDEX IF NOT EXISTS idx_orders_status ON cross_chain_orders(status)",
            "CREATE INDEX IF NOT EXISTS idx_orders_created_at ON cross_chain_orders(created_at)",
            "CREATE INDEX IF NOT EXISTS idx_orders_deadline ON cross_chain_orders(deadline)",
        ];

        for index_sql in indexes {
            sqlx::query(index_sql).execute(&self.pool).await?;
        }

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
            SET status = $1, {} = $2, updated_at = NOW()
            WHERE order_hash = $3
        "#,
            address_field
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
