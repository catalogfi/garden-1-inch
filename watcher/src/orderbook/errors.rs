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
            OrderbookError::Database(err) => write!(f, "Database error: {err}"),
            OrderbookError::Validation(msg) => write!(f, "Validation error: {msg}"),
            OrderbookError::Serialization(msg) => write!(f, "Serialization error: {msg}"),
        }
    }
}

impl From<sqlx::Error> for OrderbookError {
    fn from(err: sqlx::Error) -> Self {
        OrderbookError::Database(err)
    }
}
