use std::fmt;

#[derive(Debug)]
pub enum OrderbookError {
    Database(sqlx::Error),
    Validation(String),
    Serialization(String),
    NotFound(String),
    InvalidData(String), // Fixed typo from "InvalidDta"
}

impl fmt::Display for OrderbookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderbookError::Database(err) => write!(f, "Database error: {err}"),
            OrderbookError::Validation(msg) => write!(f, "Validation error: {msg}"),
            OrderbookError::Serialization(msg) => write!(f, "Serialization error: {msg}"),
            OrderbookError::NotFound(msg) => write!(f, "Not found: {msg}"),
            OrderbookError::InvalidData(msg) => write!(f, "Invalid data: {msg}"),
        }
    }
}

// Implement std::error::Error
impl std::error::Error for OrderbookError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            OrderbookError::Database(e) => Some(e),
            _ => None,
        }
    }
}

// From conversions
impl From<sqlx::Error> for OrderbookError {
    fn from(err: sqlx::Error) -> Self {
        OrderbookError::Database(err)
    }
}

// // For compatibility with anyhow
// impl From<OrderbookError> for anyhow::Error {
//     fn from(err: OrderbookError) -> Self {
//         anyhow::Error::new(err)
//     }
// }
