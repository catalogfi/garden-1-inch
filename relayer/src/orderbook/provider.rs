use crate::primitives::{CrossChainOrder, OrderStatus, SignedOrderInput};
use rand::Rng;
use sha2::{Digest, Sha256};
use sqlx::{Pool, Postgres, Row};
use std::fmt;

#[derive(Debug)]
pub enum OrderbookError {
    Database(sqlx::Error),
    Validation(String),
    Serialization(String),
}

impl fmt::Display for OrderbookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderbookError::Database(err) => write!(f, "Database error: {}", err),
            OrderbookError::Validation(msg) => write!(f, "Validation error: {}", msg),
            OrderbookError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for OrderbookError {}

impl From<sqlx::Error> for OrderbookError {
    fn from(err: sqlx::Error) -> Self {
        OrderbookError::Database(err)
    }
}

impl From<serde_json::Error> for OrderbookError {
    fn from(err: serde_json::Error) -> Self {
        OrderbookError::Serialization(err.to_string())
    }
}

#[derive(Clone)]
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

    /// Drop the existing orders table and all its indexes
    pub async fn drop_tables(&self) -> Result<(), OrderbookError> {
        sqlx::query("DROP TABLE IF EXISTS orders CASCADE")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Create the orders table with all required fields and indexes
    pub async fn create_tables_with_new_schema(&self) -> Result<(), OrderbookError> {
        // Create the main table
        let create_table_sql = r#"
            CREATE TABLE orders (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                order_hash VARCHAR(66) UNIQUE NOT NULL,
                src_chain_id BIGINT NOT NULL,
                dst_chain_id BIGINT NOT NULL,
                maker VARCHAR(42) NOT NULL,
                receiver VARCHAR(42) NOT NULL,
                taker VARCHAR(42) NOT NULL,
                timelock VARCHAR(255) NOT NULL,
                maker_asset VARCHAR(42) NOT NULL,
                taker_asset VARCHAR(42) NOT NULL,
                making_amount NUMERIC NOT NULL,
                taking_amount NUMERIC NOT NULL,
                salt VARCHAR(255) NOT NULL,
                maker_traits VARCHAR(255) NOT NULL DEFAULT '0',
                taker_traits VARCHAR(255) NOT NULL DEFAULT '0',
                args JSONB NOT NULL DEFAULT '{}'::jsonb,
                signature JSONB NOT NULL,
                extension JSONB NOT NULL,
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

        // Create indexes
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_orders_maker ON orders(maker)",
            "CREATE INDEX IF NOT EXISTS idx_orders_taker ON orders(taker)",
            "CREATE INDEX IF NOT EXISTS idx_orders_chain ON orders(src_chain_id)",
            "CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status)",
            "CREATE INDEX IF NOT EXISTS idx_orders_created_at ON orders(created_at)",
            "CREATE INDEX IF NOT EXISTS idx_orders_unmatched ON orders(status) WHERE status = 'unmatched'",
            "CREATE INDEX IF NOT EXISTS idx_orders_deadline ON orders(deadline)",
        ];

        for index_sql in indexes {
            sqlx::query(index_sql).execute(&self.pool).await?;
        }

        Ok(())
    }

    /// Generate a unique order hash as 32-byte hex string
    fn generate_order_hash(&self, _signed_order: &SignedOrderInput) -> String {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill(&mut bytes);
        hex::encode(bytes)
    }

    /// Insert a new cross chain order into the database
    pub async fn create_order(
        &self,
        signed_order: &SignedOrderInput,
    ) -> Result<String, OrderbookError> {
        // Generate order hash
        // Serialize secrets to JSON
        let secrets_json = serde_json::to_value(signed_order.secrets.clone())?;
        let extension_json = serde_json::to_value(signed_order.extension.clone())?;
        let signature_json = serde_json::to_value(signed_order.signature.clone())?;
        let insert_sql = r#"
            INSERT INTO orders (
                order_hash, src_chain_id, dst_chain_id, maker, receiver, taker, timelock,
                maker_asset, taker_asset, making_amount, taking_amount,
                salt, maker_traits, taker_traits, args, signature, extension, order_type, secrets, status, deadline
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
            ON CONFLICT (order_hash) DO NOTHING
            RETURNING id
        "#;

        let result = sqlx::query(insert_sql)
            .bind(&signed_order.order_hash)
            .bind(signed_order.src_chain_id as i64)
            .bind(signed_order.dst_chain_id as i64)
            .bind(&signed_order.order.maker)
            .bind(&signed_order.order.receiver)
            .bind(&signed_order.taker)
            .bind(&signed_order.timelock)
            .bind(&signed_order.order.maker_asset)
            .bind(&signed_order.order.taker_asset)
            .bind(&signed_order.order.making_amount)
            .bind(&signed_order.order.taking_amount)
            .bind(&signed_order.order.salt)
            .bind(&signed_order.order.maker_traits)
            .bind(&signed_order.taker_traits)
            .bind(&signed_order.args)
            .bind(&signature_json)
            .bind(&extension_json)
            .bind(&signed_order.order_type.to_string())
            .bind(&secrets_json)
            .bind("unmatched")
            .bind(signed_order.deadline as i64)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(row) => Ok(row.get::<uuid::Uuid, _>("id").to_string()),
            None => Err(OrderbookError::Validation(
                "Order already exists".to_string(),
            )),
        }
    }

    /// Get an order by its ID or hash
    pub async fn get_order(
        &self,
        order_id: &str,
    ) -> Result<Option<CrossChainOrder>, OrderbookError> {
        let query_sql = r#"
            SELECT 
                id, order_hash, src_chain_id, dst_chain_id, maker, receiver, taker, timelock,
                maker_asset, taker_asset, making_amount, taking_amount, salt, maker_traits, taker_traits, args,
                signature, extension, order_type, secrets, status, deadline, 
                auction_start_date,
                auction_end_date,
                src_escrow_address, dst_escrow_address, src_tx_hash,
                dst_tx_hash, filled_maker_amount, filled_taker_amount,
                created_at, updated_at
            FROM orders WHERE order_hash = $1
        "#;

        let order = sqlx::query_as::<_, CrossChainOrder>(query_sql)
            .bind(order_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(order)
    }

    /// Get orders by source chain ID
    pub async fn get_orders_by_chain(
        &self,
        src_chain_id: u64,
    ) -> Result<Vec<CrossChainOrder>, OrderbookError> {
        let query_sql = r#"
            SELECT 
                id, order_hash, src_chain_id, dst_chain_id, maker, receiver, taker, timelock,
                maker_asset, taker_asset, making_amount, taking_amount, salt, maker_traits, taker_traits, args,
                signature, extension, order_type, secrets, status, deadline, 
                auction_start_date,
                auction_end_date,
                src_escrow_address, dst_escrow_address, src_tx_hash,
                dst_tx_hash, filled_maker_amount, filled_taker_amount,
                created_at, updated_at
            FROM orders WHERE src_chain_id = $1 ORDER BY created_at DESC
        "#;

        let orders = sqlx::query_as::<_, CrossChainOrder>(query_sql)
            .bind(src_chain_id as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(orders)
    }

    /// Get active (unmatched) orders with pagination and filtering
    pub async fn get_active_orders(
        &self,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<CrossChainOrder>, u64), OrderbookError> {
        // Get orders with pagination
        let orders = sqlx::query_as::<_, CrossChainOrder>(
            r#"
            SELECT 
                id, order_hash, src_chain_id, dst_chain_id, maker, receiver, taker, timelock,
                maker_asset, taker_asset, making_amount, taking_amount, salt, maker_traits, taker_traits, args,
                signature, extension, order_type, secrets, status, deadline, 
                auction_start_date,
                auction_end_date,
                src_escrow_address, dst_escrow_address, src_tx_hash,
                dst_tx_hash, filled_maker_amount, filled_taker_amount,
                created_at, updated_at
            FROM orders WHERE status = 'unmatched' ORDER BY created_at DESC LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        // Get total count
        let total_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM orders WHERE status = 'unmatched'")
                .fetch_one(&self.pool)
                .await?;

        Ok((orders, total_count as u64))
    }

    /// Submit a secret for an order
    pub async fn submit_secret(
        &self,
        order_hash: &str,
        secret: &str,
    ) -> Result<(), OrderbookError> {
        // First, check if the order exists and get its status
      
        // Get the current secrets for the order
        let current_secrets_result: Result<serde_json::Value, sqlx::Error> =
            sqlx::query_scalar("SELECT secrets FROM orders WHERE order_hash = $1")
                .bind(order_hash)
                .fetch_one(&self.pool)
                .await;

        let current_secrets = match current_secrets_result {
            Ok(secrets) => secrets,
            Err(sqlx::Error::RowNotFound) => {
                return Err(OrderbookError::Validation("Order not found".to_string()));
            }
            Err(e) => return Err(OrderbookError::Database(e)),
        };

        // Parse the current secrets array
        let mut secrets_array = if let serde_json::Value::Array(arr) = current_secrets {
            arr
        } else {
            vec![]
        };

        // Find the next available index
        let next_index = secrets_array.len() as u32;

        // Create a new secret entry
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        let result = hasher.finalize();
        let secret_entry = serde_json::json!({
            "index": next_index,
            "secret": secret,
            "secretHash": hex::encode(result)
        });

        // Add the new secret entry to the array
        secrets_array.push(secret_entry);

        // Update the order with the new secrets array
        let updated_rows =
            sqlx::query("UPDATE orders SET secrets = $1, updated_at = NOW() WHERE order_hash = $2")
                .bind(&serde_json::Value::Array(secrets_array))
                .bind(order_hash)
                .execute(&self.pool)
                .await?;

        if updated_rows.rows_affected() == 0 {
            return Err(OrderbookError::Validation("Order not found".to_string()));
        }

        Ok(())
    }

    /// Get all secrets for an order
    pub async fn get_secrets(&self, order_hash: &str) -> Result<Vec<String>, OrderbookError> {
        let secrets_result: Result<serde_json::Value, sqlx::Error> =
            sqlx::query_scalar("SELECT secrets FROM orders WHERE order_hash = $1")
                .bind(order_hash)
                .fetch_one(&self.pool)
                .await;

        let secrets = match secrets_result {
            Ok(secrets) => secrets,
            Err(sqlx::Error::RowNotFound) => {
                return Err(OrderbookError::Validation("Order not found".to_string()));
            }
            Err(e) => return Err(OrderbookError::Database(e)),
        };

        // Parse the secrets array and return all secrets
        if let serde_json::Value::Array(arr) = secrets {
            let mut secrets_vec = Vec::new();
            for secret_entry in arr {
                if let serde_json::Value::Object(obj) = secret_entry {
                    if let Some(serde_json::Value::String(secret)) = obj.get("secret") {
                        if secret != "null" {
                            secrets_vec.push(secret.clone());
                        }
                    }
                }
            }
            return Ok(secrets_vec);
        }

        Ok(Vec::new())
    }
}
