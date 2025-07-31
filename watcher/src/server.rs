use std::sync::Arc;

use axum::{Router, http::Method, routing::get};
use tower_http::cors::{AllowHeaders, Any};
use tracing::info;

use crate::{config::WatcherConfig, handlers::health};

/// HTTP server for the Starknet Watcher service
///
/// Provides health check endpoints
pub struct Server {
    /// Address to bind the server to (e.g., "0.0.0.0:6060")
    address: String,
    /// Configuration reference for potential future status checks
    _config: Arc<WatcherConfig>,
}

impl Server {
    /// Creates a new Server instance
    ///
    /// # Arguments
    /// * `address` - The address to bind the server to
    /// * `config` - Shared configuration reference
    pub fn new(address: String, config: Arc<WatcherConfig>) -> Self {
        Self {
            address,
            _config: config,
        }
    }

    /// Starts the HTTP server
    ///
    /// Configures CORS, sets up routes, and begins listening for requests
    pub async fn run(&self) {
        // Configure CORS settings
        let cors = tower_http::cors::CorsLayer::new()
            .allow_methods([Method::GET])
            .allow_origin(Any)
            .allow_headers(AllowHeaders::any());

        // Set up the router with health check endpoint
        let app = Router::new().route("/health", get(health)).layer(cors);

        // Bind to the specified address
        let listener = tokio::net::TcpListener::bind(&self.address).await.unwrap();

        info!("Server started at {}", self.address);

        // Start serving requests
        axum::serve(listener, app).await.unwrap();
    }
}
