pub mod handlers;

use crate::orderbook::OrderbookProvider;
use crate::server::handlers::HandlerState;
use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use handlers::{
    get_active_orders, get_health, get_order, get_orders_by_chain, get_secret, submit_order,
    submit_secret, update_order_field,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tracing::info;

/// Status of an API response
///
/// Used to indicate whether an API call was successful or encountered an error.
/// This is included as a top-level field in every response.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    /// Operation completed successfully
    Ok,
    /// Operation encountered an error
    Error,
}

/// Standard API response wrapper
///
/// This structure wraps all API responses to provide a consistent format:
/// - `status`: Indicates if the request was successful or encountered an error
/// - `data`: Contains the actual response data when successful
/// - `error`: Contains error details when the request fails
///
/// # Examples
///
/// ```
/// # use crate::api::primitives::{Response, Status};
/// let success = Response::ok("success data");
/// let error = Response::<()>::error("something went wrong");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Response<T> {
    /// Status of the response (Ok or Error)
    pub status: Status,

    /// The response payload when status is Ok
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,

    /// Error details when status is Error
    /// Only present when an error occurs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// The status code of the response
    #[serde(skip)]
    pub status_code: StatusCode,
}

impl<T> Response<T> {
    /// Creates a successful response with the given data
    ///
    /// # Arguments
    ///
    /// * `data` - The data to include in the successful response
    ///
    /// # Returns
    ///
    /// A JSON-wrapped Response with Ok status and the provided data
    pub fn ok(data: T) -> Self {
        Self {
            status: Status::Ok,
            result: Some(data),
            error: None,
            status_code: StatusCode::OK,
        }
    }

    /// Creates an error response with the given error message
    ///
    /// # Arguments
    ///
    /// * `error` - Any type that can be converted to a String
    ///
    /// # Returns
    ///
    /// A JSON-wrapped Response with Error status and the provided error message
    pub fn error<E: ToString>(error: E, status_code: StatusCode) -> Self {
        Self {
            status: Status::Error,
            error: Some(error.to_string()),
            result: None,
            status_code,
        }
    }
}

impl<T> IntoResponse for Response<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        let mut response = Json(self).into_response();
        *response.status_mut() = status_code;
        response
    }
}

pub struct Server {
    pub port: u16,
    pub state: HandlerState,
}

impl Server {
    pub async fn new(port: u16, db_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let orderbook = OrderbookProvider::from_db_url(db_url).await?;

        orderbook.drop_tables().await?;
        orderbook.create_tables_with_new_schema().await?;

        let state = HandlerState { orderbook };

        Ok(Self { port, state })
    }

    pub async fn run(&self) {
        // Basic CORS setup
        let cors = CorsLayer::new()
            .allow_methods(vec![Method::GET, Method::POST])
            .allow_origin(Any)
            .allow_headers(AllowHeaders::any());

        // Create sub-routers
        let relayer_routes = Router::new()
            .route("/submit", post(submit_order))
            .route("/secret", post(submit_secret))
            .with_state(self.state.clone());

        let orders_routes = Router::new()
            .route("/active", get(get_active_orders))
            .route("/{order_id}", get(get_order))
            .route("/chain/{chain_id}", get(get_orders_by_chain))
            .route("/secret/{order_hash}", get(get_secret))
            .route("/update/{order_hash}", post(update_order_field))
            .with_state(self.state.clone());

        // Create main router with sub-routes
        let app = Router::new()
            .route("/health", get(get_health))
            .nest("/relayer", relayer_routes)
            .nest("/orders", orders_routes)
            .layer(cors)
            .with_state(self.state.clone());

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        info!("Listening on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
