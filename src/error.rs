//! Error types for the CryptoPay library

use thiserror::Error;

/// Result type alias for CryptoPay operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for CryptoPay operations
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),

    /// BscScan API returned an error
    #[error("BscScan API error: {message}")]
    ApiError { message: String },

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Please retry after some time")]
    RateLimitExceeded,

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Invalid address format
    #[error("Invalid address format: {0}")]
    InvalidAddress(String),

    /// Invalid transaction hash
    #[error("Invalid transaction hash: {0}")]
    InvalidTxHash(String),

    /// Transaction not found
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),

    /// Payment verification failed
    #[error("Payment verification failed: {0}")]
    VerificationFailed(String),

    /// Amount mismatch
    #[error("Amount mismatch: expected {expected}, found {actual}")]
    AmountMismatch { expected: String, actual: String },

    /// Recipient mismatch
    #[error("Recipient mismatch: expected {expected}, found {actual}")]
    RecipientMismatch { expected: String, actual: String },

    /// Token contract mismatch
    #[error("Token contract mismatch: expected {expected}, found {actual}")]
    TokenMismatch { expected: String, actual: String },

    /// Insufficient confirmations
    #[error("Insufficient confirmations: {current}/{required}")]
    InsufficientConfirmations { current: u64, required: u64 },

    /// Payment timeout
    #[error("Payment timeout: no transaction found within {0} seconds")]
    PaymentTimeout(u64),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Cache error
    #[error("Cache error: {0}")]
    CacheError(String),

    /// Storage error
    #[cfg(any(feature = "postgres-storage", feature = "sqlite-storage"))]
    #[error("Storage error: {0}")]
    StorageError(#[from] sqlx::Error),

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

impl Error {
    /// Create a new API error
    pub fn api_error(message: impl Into<String>) -> Self {
        Self::ApiError {
            message: message.into(),
        }
    }

    /// Create a new verification failed error
    pub fn verification_failed(message: impl Into<String>) -> Self {
        Self::VerificationFailed(message.into())
    }

    /// Create a new generic error
    pub fn generic(message: impl Into<String>) -> Self {
        Self::Generic(message.into())
    }
}
